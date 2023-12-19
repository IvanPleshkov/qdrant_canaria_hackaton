use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::JoinHandle,
};

use crate::{action::ActionsCollection, embeddings_generator::EmbeddingsGenerator, qdrant::QDrant};

pub fn start_promt_processor(
    user_promt: Arc<Mutex<String>>,
    receiver: Receiver<()>,
) -> JoinHandle<()> {
    let embeddings_generator = EmbeddingsGenerator::new();
    let actions_collection = ActionsCollection::new();
    let process_qdrant = QDrant::new().unwrap();
    let mut history_qdrant = QDrant::new().unwrap();

    std::thread::spawn(move || {
        while let Ok(_) = receiver.recv() {
            let mut user_promt = user_promt.lock().unwrap();
            log::debug!("PROMT: {}", user_promt);

            let embedding = embeddings_generator.generate(&user_promt).unwrap();
            history_qdrant.insert(embedding.clone()).unwrap();

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
