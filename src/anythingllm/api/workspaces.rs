//! AnythingLLM API 'Workspaces' endpoints

use serde_json::json;

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::error::LLMError;
use crate::anythingllm::error::LLMError::WorkspaceIdError;
use crate::anythingllm::models::workspace::{WorkspaceData, WorkspacesResponse};

impl AnythingLLMClient {
    /// Create a new workspace
    pub async fn workspace_create(&self, name: &str) -> Result<WorkspaceData, LLMError> {
        self.post_name("workspace/new", name).await
    }

    /// Delete a workspace
    pub async fn workspace_delete(&self, slug: &str) -> Result<(), LLMError> {
        self.delete("workspace", slug).await
    }

    /// Get all workspaces
    pub async fn workspace_list(&self) -> Result<Vec<WorkspaceData>, LLMError> {
        let response = self.get::<WorkspacesResponse>("workspaces").await?;
        Ok(response.workspaces)
    }

    /// Add a document to a workspace
    pub async fn workspace_update_embeddings(
        &self,
        workspace_slug: &str,
        document_slugs: Vec<&str>,
        update_parameter: UpdateParameter,
    ) -> Result<(), LLMError> {
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
    pub async fn workspace_slug_from_id(&self, id: u8) -> Result<String, LLMError> {
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
    use std::env;

    use dotenv::dotenv;

    use super::*;

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
}
