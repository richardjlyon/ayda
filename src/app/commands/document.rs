//! Document commands
//!

use colored::*;
use eyre::Context;

use crate::app::commands;

/// List all documents in a workspace
///
pub async fn list() -> eyre::Result<()> {
    let client = commands::anythingllm_client();
    let documents = client
        .get_documents()
        .await
        .wrap_err("could get documents")?;

    for doc in documents {
        if let Some(title) = doc.title.clone() {
            println!("- {}", title);
        } else {
            println!("- {}", "Untitled".red());
        }
    }

    Ok(())
}
