//! Examine the JSON returned by endpoints
#![allow(dead_code)]

use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    Ok(())
}

// Client for the AnythingLLM API //////////////////////////////////////////////////////////////////

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    authenticated: bool,
}

impl Client {
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

    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, LLMError> {
        let response = self.get("workspaces").await?.error_for_status()?;
        let workspaces_response = response.json::<GetWorkspacesResponse>().await?;

        Ok(workspaces_response.workspaces)
    }

    pub async fn get_workspace_slug(&self, slug: &str) -> Result<Workspace, LLMError> {
        let endpoint = format!("{}/{}", "workspace", slug);
        let response = self.get(&endpoint).await?.error_for_status()?;

        let workspace_slug_response = response.json::<GetWorkspaceSlugResponse>().await?;

        Ok(workspace_slug_response.workspace)
    }

    pub async fn post_workspace_new(&self, name: &str) -> Result<Workspace, LLMError> {
        let response = self
            .client
            .post(&format!("{}/{}", self.base_url, "workspace/new"))
            .header("Content-Type", "application/json")
            .body(json!({ "name": name }).to_string())
            .send()
            .await?
            .error_for_status()?;

        let workspace_new_response = response.json::<GetWorkspaceNewResponse>().await?;

        Ok(workspace_new_response.workspace)
    }

    pub async fn delete_workspace_slug(&self, slug: &str) -> Result<(), LLMError> {
        let url = format!("{}/{}/{}", self.base_url, "workspace", slug);
        let response = self.client.delete(&url).send().await?.error_for_status()?;

        // NOTE: For a bad request, the API returns a "200 OK" status with the text "Bad Request"
        // not a "400 Bad Request"
        let response_text = response.text().await?;

        if response_text == "Bad Request" {
            return Err(LLMError::BadRequest(url));
        }

        Ok(())
    }

    async fn get(&self, endpoint: &str) -> Result<Response, LLMError> {
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

// Structs for deserializing JSON //////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Deserialize)]
struct GetWorkspaceNewResponse {
    workspace: Workspace,
    message: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct GetWorkspacesResponse {
    workspaces: Vec<Workspace>,
}

#[derive(Debug, serde::Deserialize)]
struct GetWorkspaceSlugResponse {
    workspace: Workspace,
}

#[derive(Debug, serde::Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

fn print_json(json: &Value) {
    let pretty_json = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", pretty_json);
}

// Errors //////////////////////////////////////////////////////////////////////////////////////////

#[derive(thiserror::Error, Debug)]
pub enum LLMError {
    #[error("Authentication error")]
    AuthError,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Service error: {0}")]
    ServiceError(#[from] reqwest::Error),
    #[error("Unhandled error")]
    UnhandledError,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture {
        client: Client,
        workspace: Workspace,
    }

    impl Fixture {
        async fn new() -> Self {
            dotenv::dotenv().ok();
            let api_key = &env::var("ANYTHINGLLM_API_KEY").expect("API key not found");
            let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
            let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
            let client = Client::new(ip, port, api_key);
            let uuid = uuid::Uuid::new_v4();
            let workspace_name = format!("DELETE ME {}", uuid);
            let workspace = client.post_workspace_new(&workspace_name).await.unwrap();
            Self { client, workspace }
        }

        async fn remove(self) {
            let _ = &self
                .client
                .delete_workspace_slug(&self.workspace.slug)
                .await;
        }
    }

    #[tokio::test]
    async fn test_client_new() {
        dotenv::dotenv().ok();
        let api_key = "api_key";
        let ip = "10.13.10.8";
        let port = "3001";
        let client = Client::new(ip, port, api_key);

        assert_eq!(client.base_url, "http://10.13.10.8:3001/api/v1");
    }

    #[tokio::test]
    async fn test_auth_ok() {
        dotenv::dotenv().ok();
        let api_key = &env::var("ANYTHINGLLM_API_KEY").expect("API key not found");
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = Client::new(ip, port, api_key);

        assert!(client.get_auth().await.is_ok());
    }

    #[tokio::test]
    async fn test_auth_err() {
        dotenv::dotenv().ok();
        let api_key = "INVALID_API_KEY";
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = Client::new(ip, port, api_key);

        assert!(client.get_auth().await.is_err());
    }

    #[tokio::test]
    async fn test_post_workspace_new() {
        let fixture = Fixture::new().await;
        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_delete_workspace_slug() {
        let fixture = Fixture::new().await;
        let test_workspace_slug = &fixture.workspace.slug;
        let _ = fixture
            .client
            .delete_workspace_slug(&test_workspace_slug)
            .await
            .unwrap();
        let workspaces = fixture.client.get_workspaces().await.unwrap();

        assert!(!workspaces
            .iter()
            .any(|w| w.slug == test_workspace_slug.to_string()));
    }

    #[tokio::test]
    async fn test_get_workspaces() {
        let fixture = Fixture::new().await;
        let workspaces = fixture.client.get_workspaces().await.unwrap();
        let workspace_slug = &fixture.workspace.slug;

        assert!(workspaces.len() > 0);
        assert!(workspaces
            .iter()
            .any(|w| w.slug == workspace_slug.to_string()));

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_workspace_slug() {
        let fixture = Fixture::new().await;
        let test_workspace_slug = &fixture.workspace.slug;
        let workspace = fixture
            .client
            .get_workspace_slug(&test_workspace_slug)
            .await
            .unwrap();

        assert_eq!(workspace.slug, test_workspace_slug.to_string());

        fixture.remove().await;
    }
}
