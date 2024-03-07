use crate::app::commands;
use crate::app::error::AppError::{DocumentMissingError, WorkspaceIdError};
use crate::app::error::Result;

pub async fn document_add(document_filepath: &str, workspace_id: u8) -> Result<()> {
    // Check the document exists
    if !std::path::Path::new(document_filepath).exists() {
        return Err(DocumentMissingError(document_filepath.to_string()));
    }

    let client = commands::anythingllm_client();

    // Check the workspace exists
    let workspace_slug = client
        .workspace_list()
        .await?
        .iter()
        .find(|ws| ws.id == workspace_id)
        .ok_or(WorkspaceIdError(workspace_id))?
        .slug
        .clone();

    client.document_add(&document_filepath).await?;

    // Add it to the workspace

    Ok(())
}
