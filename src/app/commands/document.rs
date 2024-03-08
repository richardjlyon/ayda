//! Document commands
//!
use crate::anythingllm::api::workspaces::UpdateParameter;
use crate::app::commands;
use crate::app::error::Result;

/// Add a document to a workspace
pub async fn document_add(document_filepath: &str, workspace_id: u8) -> Result<()> {
    let client = commands::anythingllm_client();
    let document = client.document_add(document_filepath).await?;
    let workspace_slug = client.workspace_slug_from_id(workspace_id).await?;

    client
        .workspace_update_embeddings(
            &workspace_slug,
            vec![&document.doc_filepath_internal()],
            UpdateParameter::Adds,
        )
        .await?;

    Ok(())
}
