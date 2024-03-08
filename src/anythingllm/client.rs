use std::fs;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{multipart, Client, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

/// The `AnythingLLMClient` struct represents a client for the AnythingLLM API.
/// It includes the base URL for the API and a `reqwest::Client` for making requests.
use crate::anythingllm::error::LLMError;
use crate::anythingllm::models::document::DocumentUploadResponse;
use crate::anythingllm::models::workspace::{WorkspaceData, WorkspaceNewResponse};

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

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, LLMError> {
        let response = self
            .client
            .get(format!("{}/{}", self.base_url, endpoint))
            .send()
            .await?;

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

    pub async fn delete(&self, endpoint: &str, slug: &str) -> Result<(), LLMError> {
        let url = format!("{}/{}/{}", self.base_url, endpoint, slug);
        let response = self.client.delete(&url).send().await?;

        match response.status() {
            StatusCode::OK => Ok(()),
            e => Err(LLMError::ServiceError(e.to_string())),
        }
    }

    pub async fn post_name(&self, endpoint: &str, name: &str) -> Result<WorkspaceData, LLMError> {
        let response = self
            .client
            .post(&format!("{}/{}", self.base_url, endpoint))
            .header("Content-Type", "application/json")
            .body(json!({ "name": name }).to_string())
            .send()
            .await?;

        response
            .json::<WorkspaceNewResponse>()
            .await
            .map(|data| data.workspace)
            .map_err(|e| LLMError::ServiceError(e.to_string()))
    }

    pub async fn post_multipart(
        &self,
        endpoint: &str,
        form: multipart::Form,
    ) -> Result<DocumentUploadResponse, LLMError> {
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

        let document_response: DocumentUploadResponse = response.json().await?;
        Ok(document_response)
    }

    pub async fn post_json(
        &self,
        endpoint: &str,
        json_body: serde_json::Value,
    ) -> Result<(), LLMError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(&json_body).send().await?;

        match response.status() {
            StatusCode::OK => Ok(()),
            e => Err(LLMError::ServiceError(e.to_string())),
        }
    }

    // TEST

    pub async fn post_json_raw(
        &self,
        endpoint: &str,
        json_body: serde_json::Value,
    ) -> Result<(), LLMError> {
        let response = self
            .client
            .post(&format!("{}/{}", self.base_url, endpoint))
            .json(&json_body)
            .send()
            .await?;

        let data = response.text().await?;
        let json: Value = serde_json::from_str(&data).unwrap();
        let pretty_json = serde_json::to_string_pretty(&json).unwrap();
        fs::write("document_upload.json", &pretty_json).expect("Unable to write file");

        Ok(())
    }

    // END TEST
}

#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;

    use crate::anythingllm::models::workspace::WorkspacesResponse;

    use super::*;

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

        match client.get::<WorkspacesResponse>("workspaces").await {
            Ok(_) => panic!("Expected an error, but got a successful response"),
            Err(err) => match err {
                LLMError::AuthFail(_) => (), // Test passes if we get here
                _ => panic!("Expected AuthFail, but got a different error"),
            },
        }
    }
}
