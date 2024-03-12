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

    /// DELETE /workspace/{slug}
    pub async fn delete_workspace_slug(&self, slug: &str) -> Result<(), LLMError> {
        let url = format!("{}/{}/{}", self.base_url, "workspace", slug);
        let response = match self.client.delete(&url).send().await {
            Ok(response) => response,
            Err(e) => return Err(LLMError::ServiceError(e.to_string())),
        };

        // NOTE: For a bad request, the API returns a "200 OK" status with the text "Bad Request"
        // not a "400 Bad Request"
        let response_text = response.text().await?;

        if response_text == "Bad Request" {
            return Err(LLMError::BadRequest(url));
        }

        Ok(())
    }
}
