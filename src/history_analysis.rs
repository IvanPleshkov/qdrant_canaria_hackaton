use serde::{Deserialize, Serialize};

use crate::{embeddings_generator::EmbeddingsGenerator, qdrant::QDrant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub target: String,
    pub limit: usize,
    pub actions: Vec<String>,
    pub start_phrase: Option<String>,
}

pub fn run(filename: &str) {
    let input_data = std::fs::read_to_string(filename).unwrap();
    let input: Input = serde_json::from_str(&input_data).unwrap();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build();

    runtime.unwrap().block_on(async move {
        let embeddings_generator = EmbeddingsGenerator::new();
        let qdrant = QDrant::new().await.unwrap();

        let start_phrase = input
            .start_phrase
            .unwrap_or_else(|| "Hello world".to_string());
        let mut target = embeddings_generator.generate(&start_phrase).await.unwrap();
        target = target.iter().map(|x| -x).collect();

        let positive = embeddings_generator.generate(&input.target).await.unwrap();
        let mut negatives = embeddings_generator
            .generate_many(&input.actions)
            .await
            .unwrap();
        let mut positives: Vec<_> = (0..negatives.len()).map(|_| positive.clone()).collect();

        for _ in 0..input.limit {
            let discover_result = qdrant
                .search_discovery(&target, 1, positives.clone(), negatives.clone())
                .await
                .unwrap();
            let (discovered_text, discovered_vector) = discover_result.first().unwrap();
            log::info!("{}", discovered_text);

            positives.push(positive.clone());
            negatives.push(discovered_vector.clone());

            // completely new target with opposite direction
            target = discovered_vector.iter().map(|x| -x).collect();
        }
    });
}
