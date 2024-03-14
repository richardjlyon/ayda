//! Anythingllm client module

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::multipart::Form;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::anythingllm::error::LLMError;

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    authenticated: bool,
}

/// An AnythingLLM client.
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

    pub async fn post(&self, endpoint: &str, body: &Value) -> Result<Response, LLMError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .post(url.clone())
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?
            .error_for_status()?;

        Ok(response)
    }

    /// Get the authentication status from the AnythingLLM instance.
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

    pub async fn post_multipart(&self, endpoint: &str, form: Form) -> Result<Response, LLMError> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = self
            .client
            .post(url.clone())
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        Ok(response)
    }

    // /// For debugging response
    // pub async fn post_multipart_raw(&self, endpoint: &str, form: Form) -> Result<(), LLMError> {
    //     let url = format!("{}/{}", self.base_url, endpoint);
    //
    //     // -- DEBUG
    //
    //     let response = self
    //         .client
    //         .post(url.clone())
    //         .multipart(form)
    //         .send()
    //         .await?
    //         .error_for_status()?;
    //
    //     // let body_text = response.text().await?;
    //     let body_json = response.json::<serde_json::Value>().await?;
    //     let json_pretty = serde_json::to_string_pretty(&body_json).unwrap();
    //     std::fs::write("../../tests/responses/AnythingLLM/post_multipart_raw.json", json_pretty).unwrap();
    //     // dbg!(&body_json);
    //
    //     Ok(())
    // }
}
