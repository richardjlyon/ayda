use crate::error::{LLMError, Result};
/// The `AnythingLLMClient` struct represents a client for the AnythingLLM API.
/// It includes the base URL for the API and a `reqwest::Client` for making requests.
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{multipart, Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::json;

pub struct AnythingLLMClient {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl AnythingLLMClient {
    pub fn new(server_ip: &str, port: &str, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );

        Self {
            base_url: format!("http://{}:{}/api/v1", server_ip, port),
            client: Client::builder().default_headers(headers).build().unwrap(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;

        match response.status() {
            StatusCode::FORBIDDEN => Err(LLMError::AuthFail("Unauthorized".to_string())),
            StatusCode::INTERNAL_SERVER_ERROR => {
                Err(LLMError::ServiceError("Internal Server Error".to_string()))
            }
            _ => response
                .json::<T>()
                .await
                .map_err(|e| LLMError::ServiceError(e.to_string())),
        }
    }

    pub async fn post(&self, endpoint: &str, name: &str) -> Result<()> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let body = json!({
            "name": name
        });

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(()),
            e => Err(LLMError::ServiceError(e.to_string())),
        }
    }

    pub async fn post_multipart(
        &self,
        endpoint: &str,
        form: multipart::Form,
    ) -> Result<DocumentResponse> {
        let response = self
            .client
            .post(&format!("{}/{}", self.base_url, endpoint))
            .multipart(form)
            .send()
            .await
            .unwrap();

        if response.status() != StatusCode::OK {
            return Err(LLMError::ServiceError(response.status().to_string()));
        }

        let document_response: DocumentResponse = response.json().await?;
        Ok(document_response)
    }
}

#[derive(Debug, Deserialize)]
pub struct DocumentResponse {
    pub success: bool,
    pub documents: Vec<DocumentDetails>,
}

#[derive(Debug, Deserialize)]
pub struct DocumentDetails {
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::anythingllm::api::workspaces::WorkspaceResponse;
    use dotenv::dotenv;
    use std::env;

    #[test]
    fn new() {
        let a = AnythingLLMClient::new("10.13.10.8", "3001", "api_key");
        assert_eq!(a.base_url, "http://10.13.10.8:3001/api/v1");
    }

    #[tokio::test]
    async fn test_auth_fail() {
        dotenv().ok();
        let client = AnythingLLMClient::new(
            &env::var("ANYTHINGLLM_IP").expect("IP not found"),
            &env::var("ANYTHINGLLM_PORT").expect("port not found"),
            "invalid_api_key",
        );

        match client.get::<WorkspaceResponse>("workspaces").await {
            Ok(_) => panic!("Expected an error, but got a successful response"),
            Err(err) => match err {
                LLMError::AuthFail(_) => (), // Test passes if we get here
                _ => panic!("Expected AuthFail, but got a different error"),
            },
        }
    }
}
