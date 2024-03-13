//! Document commands
//!
//! Document commands
//!
use colored::*;
use eyre::Context;

use crate::app::commands;

/// List all documents in a workspace
///
pub async fn document_list() -> eyre::Result<()> {
    let client = commands::anythingllm_client();
    let documents = client
        .get_documents()
        .await
        .wrap_err("could get documents")?;

    println!("{}", "Listing all documents".green());
    // enumerate the documents and generate an index

    for (idx, doc) in documents.iter().enumerate() {
        if let Some(title) = doc.title.clone() {
            println!("{:>2}: {}", idx + 1, title);
        } else {
            println!("{:>2}: {}", idx + 1, "Untitled".red());
        }
    }

    Ok(())
}

/// Add a document to a workspace
///
pub async fn document_upload(document_filepath: &str) -> eyre::Result<()> {
    let client = commands::anythingllm_client();
    let document = client
        .post_document_upload(document_filepath)
        .await
        .wrap_err("FIXME Failed to add document")?;

    println!("Document added: {:?}", document);

    // let workspace_slug = client.workspace_slug_from_id(workspace_id)
    //     .await
    //     .wrap_err("couldn't get workspace slug from id")?;

    // client
    //     .workspace_update_embeddings(
    //         &workspace_slug,
    //         vec![&document.doc_filepath_internal()],
    //         UpdateParameter::Adds,
    //     )
    //     .await?;
    //
    // println!(
    //     "{} {}",
    //     "Document added to workspace".green(),
    //     workspace_slug.to_string().green().bold()
    // );

    Ok(())
}
