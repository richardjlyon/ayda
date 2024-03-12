use serde_json::json;

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::error::LLMError;
use crate::anythingllm::workspace::{GetWorkspaceNewResponse, GetWorkspacesResponse, Workspace};

impl AnythingLLMClient {
    /// GET /workspaces
    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, LLMError> {
        let response = self.get("workspaces").await?.error_for_status()?;

        let workspaces_response = response.json::<GetWorkspacesResponse>().await?;

        Ok(workspaces_response.workspaces)
    }

    /// POST /workspace/new
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
}
