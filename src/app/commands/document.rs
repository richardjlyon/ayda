//! Document commands
//!
use crate::anythingllm::api::workspaces::UpdateParameter;
use crate::app::commands;
use crate::app::error::Result;
use colored::*;

/// List all documents in a workspace
pub async fn document_list() -> Result<()> {
    let client = commands::anythingllm_client();
    let documents = client.document_list().await?;

    println!("{}", "Listing all documents".green());
    // enumerate the documents and generate an index

    for (idx, doc) in documents.iter().enumerate() {
        println!("{:>2}: {}", idx + 1, doc.title);
    }

    Ok(())
}

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

    println!(
        "{} {}",
        "Document added to workspace".green(),
        workspace_slug.to_string().green().bold()
    );

    Ok(())
}

// pub async fn document_remove(document_id: u8) -> Result<()> {
//     let client = commands::anythingllm_client();
//     let documents = client.document_list().await?;
//     let document = client.document_remove(document_id).await?;
//
//     println!("{} {}", "Document removed".green(), document.title.green().bold());
//
//     Ok(())
// }
