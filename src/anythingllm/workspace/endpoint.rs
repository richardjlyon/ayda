use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::json;

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::error::LLMError;
use crate::anythingllm::workspace::*;
use crate::app::commands::workspace::import::UpdateParameter;

/// Specifies the chat mode.
#[derive(Debug, Clone)]
pub enum ChatMode {
    /// Will not use LLM unless there are relevant sources from vectorDB & does not recall chat history.
    Query,
    /// Uses LLM general knowledge w/custom embeddings to produce output, uses rolling chat history.
    Chat,
}

impl<'de> Deserialize<'de> for ChatMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "chat" => Ok(ChatMode::Chat),
            "query" => Ok(ChatMode::Query),
            _ => Err(D::Error::custom("Invalid value for ChatMode")),
        }
    }
}

impl AnythingLLMClient {
    /// POST /workspace/new
    pub async fn create_workspace(&self, name: &str) -> Result<Workspace, LLMError> {
        let response = self
            .client
            .post(&format!("{}/{}", self.base_url_api_v1, "workspace/new"))
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
    pub async fn get_workspace_by_slug(&self, slug: &str) -> Result<Workspace, LLMError> {
        let url = format!("{}/{}", "workspace", slug);

        let response = match self.get(&url).await {
            Ok(response) => response,
            Err(e) => return Err(LLMError::ServiceError(e.to_string())),
        };

        let workspace = response.json::<GetWorkspaceSlugResponse>().await?;

        workspace.workspace.ok_or(LLMError::CustomError(
            "Invalid response from server: expected struct Workspace, got null".to_string(),
        ))
    }

    /// Return a matching workspace if workspace_name corresponds to exactly one workspace
    #[tracing::instrument(skip(self))]
    pub async fn get_workspace_by_name(&self, workspace_name: &str) -> Result<Workspace, LLMError> {
        let workspaces = self.get_workspaces().await?;
        let matching_workspaces: Vec<_> = workspaces
            .iter()
            .filter(|w| w.name == workspace_name)
            .collect();

        match matching_workspaces.len() {
            0 => Err(LLMError::WorkspaceIdError(workspace_name.to_string())),
            1 => Ok(matching_workspaces[0].clone()),
            _ => Err(LLMError::MultipleWorkspacesError(
                workspace_name.to_string(),
            )),
        }
    }

    /// DELETE /workspace/{slug}
    ///
    /// Delete a workspace by its slug and remove all embedded documents from the file system.
    ///
    pub async fn delete_workspace_slug(&self, slug: &str) -> Result<(), LLMError> {
        // verify the workspace exists
        let _ = self.get_workspace_by_slug(slug).await?;

        // delete all documents in the workspace
        let documents = self.get_workspace_by_slug(slug).await?.documents.unwrap();

        let docpaths: Vec<String> = documents.iter().map(|d| d.docpath.clone()).collect();

        self.delete_api_system_remove_documents(docpaths)
            .await
            .unwrap();

        // delete the workspace
        let url = format!("{}/{}/{}", self.base_url_api_v1, "workspace", slug);
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
    pub async fn update_embeddings(
        &self,
        slug: &str,
        docs: Vec<String>,
        direction: UpdateParameter,
    ) -> Result<(), LLMError> {
        let url = format!("{}/{}/{}", "workspace", slug, "update-embeddings");
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

    /// POST /workspace/{slug}/chat
    pub async fn post_workspace_slug_chat(
        &self,
        slug: &str,
        message: &str,
        mode: &ChatMode,
    ) -> Result<PostWorkspaceSlugChatResponse, LLMError> {
        let url = format!("{}/{}/{}", "workspace", slug, "chat");
        let json = match mode {
            ChatMode::Query => json!({ "message": message, "mode": "query" }),
            ChatMode::Chat => json!({ "message": message, "mode": "chat" }),
        };

        let response = self
            .post(&url, &json)
            .await?
            .error_for_status()?
            .json::<PostWorkspaceSlugChatResponse>()
            .await?;

        Ok(response)
    }

    /// Remove all workspaces
    pub async fn delete_all_workspaces(&self) -> Result<(), LLMError> {
        let workspaces = self.get_workspaces().await?;
        for workspace in workspaces {
            self.delete_workspace_slug(&workspace.slug).await?;
        }
        Ok(())
    }
}
