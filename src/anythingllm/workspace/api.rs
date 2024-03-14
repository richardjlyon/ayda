use serde_json::json;

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::documents::Document;
use crate::anythingllm::error::LLMError;
use crate::anythingllm::workspace::{
    GetWorkspaceNewResponse, GetWorkspaceSlugResponse, GetWorkspacesResponse, Workspace,
};
use crate::app::commands::zotero::UpdateParameter;

impl AnythingLLMClient {
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

    /// GET /workspaces
    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, LLMError> {
        let response = self.get("workspaces").await?.error_for_status()?;

        let workspaces_response = response.json::<GetWorkspacesResponse>().await?;

        Ok(workspaces_response.workspaces)
    }

    /// GET / workspace/{slug}
    pub async fn get_workspace_slug(&self, slug: &str) -> Result<Workspace, LLMError> {
        let response = match self.get(format!("{}/{}", "workspace", slug).as_str()).await {
            Ok(response) => response,
            Err(e) => return Err(LLMError::ServiceError(e.to_string())),
        };

        match response.json::<GetWorkspaceSlugResponse>().await {
            Ok(workspace_slug_response) => Ok(workspace_slug_response.workspace),
            Err(err) => {
                if err.is_decode() {
                    Err(LLMError::CustomError(
                        "Invalid response from server: expected struct Workspace, got null"
                            .to_string(),
                    ))
                } else {
                    Err(LLMError::ServiceError(err.to_string()))
                }
            }
        }
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

    /// POST /workspace/{slug}/update-embeddings
    pub async fn post_workspace_slug_update_embeddings(
        &self,
        slug: &str,
        docs: Vec<Document>,
        direction: UpdateParameter,
    ) -> Result<(), LLMError> {
        let url = format!("{}/{}/{}", "workspace", slug, "update-embeddings");
        let docs: Vec<String> = docs.into_iter().filter_map(|d| d.location).collect();
        let json = match direction {
            UpdateParameter::Adds => json!({ "adds": docs }),
            UpdateParameter::Deletes => json!({ "deletes": docs }),
        };

        let response = self
            .post(&url, &json)
            .await
            .map_err(|e| LLMError::ServiceError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LLMError::ServiceError(url));
        }

        Ok(())
    }
}
