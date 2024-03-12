use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::error::LLMError;
use crate::anythingllm::workspace::{GetWorkspacesResponse, Workspace};

impl AnythingLLMClient {
    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, LLMError> {
        let response = self.get("workspaces").await?.error_for_status()?;
        let workspaces_response = response.json::<GetWorkspacesResponse>().await?;

        Ok(workspaces_response.workspaces)
    }
}
