use reqwest::Url;
use serde::{Deserialize, Serialize};

// free access key
const API_KEY: &str = "KVxr2s44nE14a3gu8rBnL8lR8VWxlCyIbaW2IdPj";
const MODEL: &str = "multilingual-22-12";

#[derive(Serialize, Deserialize)]
struct CohereRequest {
    pub texts: Vec<String>,
    pub model: String,
    pub truncate: String,
}

#[derive(Serialize, Deserialize)]
struct CohereResponse {
    pub embeddings: Vec<Vec<f32>>,
}

impl CohereRequest {
    pub fn new(texts: Vec<String>) -> Self {
        CohereRequest {
            texts,
            model: MODEL.to_string(),
            truncate: "END".to_string(),
        }
    }
}

pub struct EmbeddingsGenerator {
    client: reqwest::Client,
}

impl EmbeddingsGenerator {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }

    pub async fn generate(&self, s: &str) -> Result<Vec<f32>, String> {
        let request_body = CohereRequest::new(vec![s.to_owned()]);
        let request_body = serde_json::to_string(&request_body).unwrap();

        let resp = self
            .client
            .post(Url::parse("https://api.cohere.ai/v1/embed").unwrap())
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .body(request_body)
            .bearer_auth(API_KEY)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            let response = resp.text().await.unwrap();
            let resp: CohereResponse = serde_json::from_str(&response).unwrap();
            if let Some(embedding) = resp.embeddings.first() {
                Ok(embedding.clone())
            } else {
                let error_message = format!("Error: no embeddings in response");
                println!("{}", error_message);
                Err(error_message)
            }
        } else {
            let error_message =
                format!("Error {}: {:?}", resp.status(), resp.text().await.unwrap());
            println!("{}", error_message);
            Err(error_message)
        }
    }

    pub async fn generate_many(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, String> {
        log::info!("Generating embeddings for {} texts", texts.len());
        let mut result = vec![];
        let mut calls_counter = 0;
        let mut counter = 0;
        let data_size = texts.len();
        for texts in texts.chunks(96) {
            calls_counter += 1;
            if calls_counter == 99 {
                println!("sleep for 30 seconds");
                std::thread::sleep(std::time::Duration::from_secs_f32(35.0));
                calls_counter = 0;
            }

            counter += texts.len();
            let texts = texts.to_vec();
            let request_body = CohereRequest::new(texts);
            let request_body = serde_json::to_string(&request_body).unwrap();

            let resp = self
                .client
                .post(Url::parse("https://api.cohere.ai/v1/embed").unwrap())
                .header("accept", "application/json")
                .header("content-type", "application/json")
                .body(request_body)
                .bearer_auth(API_KEY)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if resp.status().is_success() {
                let response = resp.text().await.unwrap();
                let resp: CohereResponse = serde_json::from_str(&response).unwrap();
                for e in resp.embeddings {
                    result.push(e);
                }
            } else {
                let error = format!(
                    "Error while generating embeddings {}: {:?}",
                    resp.status(),
                    resp.text().await
                );
                log::error!("{}", &error);
                return Err(error);
            }

            println!("{} / {}", counter, data_size);
        }
        Ok(result)
    }
}
