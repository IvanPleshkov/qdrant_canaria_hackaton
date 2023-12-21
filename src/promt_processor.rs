use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::JoinHandle,
};

use crate::{
    action::ActionsCollection,
    embeddings_generator::EmbeddingsGenerator,
    qdrant::{QDrant, ScoredIndex},
    scene::Scene,
    Error, AIRPLANE_MODE,
};

pub fn init_qdrant(
    embeddings_generator: &EmbeddingsGenerator,
    actions_collection: &ActionsCollection,
) -> Result<QDrant, Error> {
    let mut qdrant = QDrant::new()?;
    let mut texts = vec![];
    for action in &actions_collection.actions {
        texts.push(action.name());
    }
    let embeddings = embeddings_generator.generate_many(&texts)?;
    for (idx, embedding) in embeddings.into_iter().enumerate() {
        qdrant.insert(idx, embedding)?;
    }
    Ok(qdrant)
}

pub fn start_promt_processor(
    user_promt: Arc<Mutex<String>>,
    scene: Arc<Mutex<Scene>>,
    receiver: Receiver<()>,
) -> JoinHandle<()> {
    let embeddings_generator = EmbeddingsGenerator::new();
    let actions_collection = ActionsCollection::new();
    log::info!("Size of actions: {}", actions_collection.actions.len());
    let process_qdrant = init_qdrant(&embeddings_generator, &actions_collection).unwrap();

    std::thread::spawn(move || {
        while let Ok(_) = receiver.recv() {
            let mut user_promt = user_promt.lock().unwrap();
            log::info!("PROMT: {}", user_promt);

            let search_result = if AIRPLANE_MODE {
                let user_promt = user_promt.to_owned().to_lowercase();
                let mut found_id = usize::MAX;
                for (i, action) in actions_collection.actions.iter().enumerate() {
                    if user_promt == action.name().to_lowercase() {
                        found_id = i;
                    }
                }
                ScoredIndex {
                    score: 0.0,
                    point: found_id,
                }
            } else {
                let embedding = embeddings_generator.generate(&user_promt).unwrap();
                process_qdrant.search(&embedding, 1).unwrap()[0]
            };

            let action = actions_collection.actions.get(search_result.point).unwrap();

            log::info!(
                "NEAREST COMMAND: {} (similarity = {})",
                &action.name(),
                search_result.score
            );
            action.execute(scene.clone()).unwrap();

            *user_promt = "".to_owned();
        }
    })
}
