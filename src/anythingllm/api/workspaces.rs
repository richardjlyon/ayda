//! AnythingLLM API 'Workspaces' endpoints

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::error::Result;
use serde::{Deserialize, Serialize};
use crate::anythingllm::models::workspace::{WorkspacesResponseWorkspace, WorkspacesResponse};

impl AnythingLLMClient {
    /// Create a new workspace
    pub async fn workspace_create(&self, name: &str) -> Result<()> {
        match self.post("workspace/new", name).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
        }
    }

    /// Delete a workspace
    pub async fn workspace_delete(&self, slug: &str) -> Result<()> {
        match self.delete("workspace", &slug).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
        }
    }

    /// Get all workspaces
    pub async fn workspace_list(&self) -> Result<Vec<WorkspacesResponseWorkspace>> {
        match self.get::<WorkspacesResponse>("workspaces").await {
            Ok(response) => Ok(response.workspaces),
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use dotenv::dotenv;
    use std::env;

    struct TestFixture {
        client: AnythingLLMClient,
    }

    impl TestFixture {
        fn new() -> Self {
            dotenv().ok();
            // Setup code here. For example, initialize the AnythingLLMClient.
            let client = AnythingLLMClient::new(
                &env::var("ANYTHINGLLM_IP").expect("IP not found"),
                &env::var("ANYTHINGLLM_PORT").expect("port not found"),
                &env::var("ANYTHINGLLM_API_KEY").expect("API key not found"),
            );
            Self { client }
        }
    }

    #[tokio::test]
    async fn test_workspace_create() {
        let fixture = TestFixture::new();
        let result = fixture
            .client
            .workspace_create("TEST WORKSPACE (DELETE)")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_workspace_list() {
        let fixture = TestFixture::new();
        let workspaces = fixture.client.workspace_list().await.unwrap();
        assert!(workspaces.len() > 0);
        println!("{:?}", workspaces);
    }

    #[tokio::test]
    async fn test_update_embeddings() {
        let fixture = TestFixture::new();

        // create a test workspace
        let today = Local::now().format("%Y-%m-%d").to_string();
        let workspace_name = format!("TEST WORKSPACE {}", today);
        fixture
            .client
            .workspace_create(&workspace_name)
            .await
            .expect("TODO: failed to create workspace");

        // upload a test document and get its slug
        let test_doc_path = "tests/test_data/2022-01-01-Test-Document.pdf";
        let doc = fixture
            .client
            .document_add(test_doc_path)
            .await
            .expect("TODO: failed to upload test document");

        // update the embeddings
        // let result = fixture.client.update_embeddings(&doc_slug).await;
        // assert!(result.is_ok());
    }
}
