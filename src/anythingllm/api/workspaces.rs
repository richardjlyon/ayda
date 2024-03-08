//! AnythingLLM API 'Workspaces' endpoints

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::error::LLMError::WorkspaceIdError;
use crate::anythingllm::error::Result;
use crate::anythingllm::models::workspace::{WorkspacesResponse, WorkspacesResponseWorkspace};

use serde_json::json;

impl AnythingLLMClient {
    /// Create a new workspace
    pub async fn workspace_create(&self, name: &str) -> Result<()> {
        self.post("workspace/new", name).await
    }

    /// Delete a workspace
    pub async fn workspace_delete(&self, slug: &str) -> Result<()> {
        self.delete("workspace", slug).await
    }

    /// Get all workspaces
    pub async fn workspace_list(&self) -> Result<Vec<WorkspacesResponseWorkspace>> {
        let response = self.get::<WorkspacesResponse>("workspaces").await?;
        Ok(response.workspaces)
    }

    /// Add a document to a workspace
    pub async fn workspace_update_embeddings(
        &self,
        workspace_slug: &str,
        document_slugs: Vec<&str>,
        update_parameter: UpdateParameter,
    ) -> Result<()> {
        let json_body = match update_parameter {
            UpdateParameter::Adds => json!({ "adds": document_slugs }),
            UpdateParameter::Deletes => json!({ "deletes": document_slugs }),
        };

        self.post_json(
            &format!("workspace/{}/update-embeddings", workspace_slug),
            json_body,
        )
        .await
    }

    /// Get a workspace slug from its id
    pub async fn workspace_slug_from_id(&self, id: u8) -> Result<String> {
        let workspaces = self.workspace_list().await?;
        let workspace = workspaces
            .iter()
            .find(|ws| ws.id == id)
            .ok_or(WorkspaceIdError(id))?;
        Ok(workspace.slug.clone())
    }
}

pub enum UpdateParameter {
    Adds,
    Deletes,
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
