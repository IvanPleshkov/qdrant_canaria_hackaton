use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::{
    action::ActionsCollection, embeddings_generator::EmbeddingsGenerator, qdrant::QDrant,
    scene::Scene, Error,
};

pub async fn init_qdrant(
    recreate_queries: bool,
    user_promt: Arc<Mutex<String>>,
    embeddings_generator: &EmbeddingsGenerator,
    actions_collection: &ActionsCollection,
) -> Result<QDrant, Error> {
    let mut _user_promt = user_promt.lock().unwrap();

    let mut qdrant = QDrant::new().await?;

    if recreate_queries {
        qdrant.recreate_queries().await?;

        let mut texts = vec![];
        for action in &actions_collection.actions {
            texts.push(action.name());
        }

        log::info!("Generating embeddings...");
        let embeddings = embeddings_generator.generate_many(&texts).await?;

        log::info!("Inserting queries...");
        for (idx, embedding) in embeddings.into_iter().enumerate() {
            if idx % 10 == 0 {
                log::info!("Inserted {} queries", idx);
            }
            qdrant.insert_query(idx, &texts[idx], embedding).await?;
        }
    }

    log::info!("Ready to go");
    Ok(qdrant)
}

pub fn start_promt_processor(
    recreate_queries: bool,
    user_promt: Arc<Mutex<String>>,
    scene: Arc<Mutex<Scene>>,
    receiver: Receiver<()>,
) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build();

        runtime.unwrap().block_on(async move {
            let embeddings_generator = EmbeddingsGenerator::new();
            let actions_collection = ActionsCollection::new();
            log::info!("Size of actions: {}", actions_collection.actions.len());
            let mut process_qdrant = init_qdrant(
                recreate_queries,
                user_promt.clone(),
                &embeddings_generator,
                &actions_collection,
            )
            .await
            .unwrap();

            while let Ok(_) = receiver.recv() {
                let mut user_promt = user_promt.lock().unwrap();
                log::info!("PROMT: {}", user_promt);

                let embedding = embeddings_generator.generate(&user_promt).await.unwrap();

                process_qdrant
                    .insert_history(&user_promt, embedding.clone())
                    .await
                    .unwrap();

                let search_result = process_qdrant.search_query(&embedding, 1).await.unwrap()[0];

                let action = actions_collection.actions.get(search_result.point).unwrap();

                log::info!(
                    "NEAREST COMMAND: {} (similarity = {})",
                    &action.name(),
                    search_result.score
                );
                action.execute(scene.clone()).unwrap();

                *user_promt = "".to_owned();
            }
        });
    });
}
