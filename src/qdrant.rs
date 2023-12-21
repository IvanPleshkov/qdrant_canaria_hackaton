use egui::ahash::HashMap;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    CreateCollection, SearchParams, SearchPoints, VectorParams, VectorsConfig,
};
use serde_json::json;

use crate::Error;

//pub const API_KEY: &str = "rWPmMZkLXmmoNqmB1Wk6vw8E5i8qfhxxTdyYPrrsjzH8gR_kOJeQFbw";
pub const API_KEY: &str = "WPmMZkLXmmoNqmB1Wk6vw8E5i8qfhxxTdyYPrrsjzH8gR_kOJeQFbw";
pub const API_ENDPOINT: &str =
    "https://7a555247-714d-4424-99b5-2312f7f119fa.us-east4-0.gcp.cloud.qdrant.io:6334";
pub const HISTORY_COLLECTION_NAME: &str = "history";
pub const QUERIES_COLLECTION_NAME: &str = "queries";
pub const LOOKUP_COLLECTION_NAME: &str = "lookup";

#[derive(Debug, Clone, Copy)]
pub struct ScoredIndex {
    pub score: f32,
    pub point: usize,
}

pub struct QDrant {
    client: QdrantClient,
}

pub struct LocalQDrant {
    pub vectors: HashMap<usize, Vec<f32>>,
}

impl QDrant {
    pub fn new() -> Result<Self, Error> {
        let client = QdrantClient::from_url(API_ENDPOINT)
            .with_api_key(API_KEY)
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Self { client })
    }

    pub async fn recreate_queries(&mut self) -> Result<(), Error> {
        let _delete = self.client.delete_collection(QUERIES_COLLECTION_NAME).await;
        self.client
            .create_collection(&CreateCollection {
                collection_name: QUERIES_COLLECTION_NAME.into(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 768,
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn insert_query(
        &mut self,
        id: usize,
        text: &str,
        vector: Vec<f32>,
    ) -> Result<(), Error> {
        let payload: Payload = json!(
            {
                "text": text.to_owned(),
            }
        )
        .try_into()
        .unwrap();
        let points = vec![PointStruct::new(id as u64, vector, payload)];
        self.client
            .upsert_points_blocking(QUERIES_COLLECTION_NAME, None, points, None)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn search_query(&self, query: &[f32], top: usize) -> Result<Vec<ScoredIndex>, Error> {
        let search_result = self
            .client
            .search_points(&SearchPoints {
                collection_name: QUERIES_COLLECTION_NAME.into(),
                vector: query.to_owned(),
                limit: top as u64,
                with_payload: Some(true.into()),
                params: Some(SearchParams {
                    exact: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;

        let mut result = vec![];
        for point in search_result.result.iter().cloned() {
            let point_id = point.id.unwrap().point_id_options.unwrap();
            let point_id = match point_id {
                point_id::PointIdOptions::Num(id) => id as usize,
                point_id::PointIdOptions::Uuid(_) => unreachable!(),
            };
            result.push(ScoredIndex {
                score: point.score,
                point: point_id,
            });
        }
        Ok(result)
    }
}
