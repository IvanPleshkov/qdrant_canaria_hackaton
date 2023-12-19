use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::JoinHandle,
};

use crate::{action::ActionsCollection, embeddings_generator::EmbeddingsGenerator, qdrant::QDrant, Error};

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
    receiver: Receiver<()>,
) -> JoinHandle<()> {
    let embeddings_generator = EmbeddingsGenerator::new();
    let actions_collection = ActionsCollection::new();
    log::info!("Size of actions: {}", actions_collection.actions.len());
    let process_qdrant = init_qdrant(
        &embeddings_generator,
        &actions_collection,
    ).unwrap();

    std::thread::spawn(move || {
        while let Ok(_) = receiver.recv() {
            let mut user_promt = user_promt.lock().unwrap();
            log::info!("PROMT: {}", user_promt);

            let embedding = embeddings_generator.generate(&user_promt).unwrap();

            let search_result = process_qdrant.search(&embedding, 1).unwrap();
            let action = actions_collection
                .actions
                .get(search_result[0].point)
                .unwrap();
            log::info!("ACTION: {}", &action.name());

            *user_promt = "".to_owned();
        }
    })
}
