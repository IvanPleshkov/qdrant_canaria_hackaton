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
    client: reqwest::blocking::Client,
}

impl EmbeddingsGenerator {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();
        Self { client }
    }

    pub fn generate(&self, s: &str) -> Result<Vec<f32>, String> {
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
            .unwrap();

        if resp.status().is_success() {
            let response = resp.text().unwrap();
            let resp: CohereResponse = serde_json::from_str(&response).unwrap();
            if let Some(embedding) = resp.embeddings.first() {
                Ok(embedding.clone())
            } else {
                let error_message = format!("Error: no embeddings in response");
                println!("{}", error_message);
                Err(error_message)
            }
        } else {
            let error_message = format!("Error {}: {:?}", resp.status(), resp.text());
            println!("{}", error_message);
            Err(error_message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embeddings_generator() {
        let embeddings_generator = EmbeddingsGenerator::new();
        let embeddings = embeddings_generator.generate("Hello world").unwrap();
        assert_eq!(embeddings.len(), 768);
    }
}