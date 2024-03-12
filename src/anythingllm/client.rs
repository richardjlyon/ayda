//! Anythingllm client module

use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::anythingllm::error::LLMError;

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    authenticated: bool,
}

pub struct AnythingLLMClient {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl AnythingLLMClient {
    pub fn new(ip: &str, port: &str, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        let base_url = format!("http://{}:{}/api/v1", ip, port);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { base_url, client }
    }

    pub async fn get_auth(&self) -> std::result::Result<bool, LLMError> {
        let response = match self.get("auth").await {
            Ok(response) => response,
            Err(_) => return Err(LLMError::AuthError),
        };

        let result = response
            .json::<AuthResponse>()
            .await
            .expect("FIXME failed to parse json");

        match result.authenticated {
            true => Ok(true),
            false => Err(LLMError::AuthError),
        }
    }

    // FIXME improve error handling to relay the error message
    pub async fn get(&self, endpoint: &str) -> Result<Response, LLMError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .get(url.clone())
            .send()
            .await?
            .error_for_status()?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_new() {
        dotenv::dotenv().ok();
        let api_key = "api_key";
        let ip = "10.13.10.8";
        let port = "3001";
        let client = AnythingLLMClient::new(ip, port, api_key);

        assert_eq!(client.base_url, "http://10.13.10.8:3001/api/v1");
    }

    #[tokio::test]
    async fn test_get_auth_ok() {
        dotenv::dotenv().ok();
        let api_key = &env::var("ANYTHINGLLM_API_KEY").expect("API key not found");
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = AnythingLLMClient::new(ip, port, api_key);

        assert!(client.get_auth().await.is_ok());
    }

    #[tokio::test]
    async fn test_get_auth_err() {
        dotenv::dotenv().ok();
        let api_key = "INVALID_API_KEY";
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = AnythingLLMClient::new(ip, port, api_key);

        assert!(client.get_auth().await.is_err());
    }
}
