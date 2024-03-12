//! Workspace commands
//!
use colored::*;

use crate::anythingllm::error::LLMError::WorkspaceIdError;
use crate::app::commands;

/// List all workspaces
///
pub async fn workspace_list() -> eyre::Result<()> {
    println!("{}", "Listing all workspaces".green());

    let client = commands::anythingllm_client();
    let workspaces = client.get_workspaces().await?;

    if workspaces.is_empty() {
        println!("{}", "No workspaces found".yellow());
    } else {
        for ws in workspaces {
            println!("{:>2}: {} ({})", ws.id, ws.name, ws.slug);
        }
    }

    Ok(())
}

/// Create a new workspace
///
pub async fn workspace_create(workspace_name: &str) -> eyre::Result<()> {
    let client = commands::anythingllm_client();
    client.post_workspace_new(workspace_name).await?;

    println!("Added workspace '{}'", workspace_name);

    Ok(())
}

/// Delete a workspace
///
pub async fn workspace_delete(workspace_id: i32) -> eyre::Result<()> {
    let client = commands::anythingllm_client();
    let workspace_slug = client
        .get_workspaces()
        .await?
        .iter()
        .find(|ws| ws.id == workspace_id)
        .ok_or(WorkspaceIdError(workspace_id))?
        .slug
        .clone();

    client.delete_workspace_slug(&workspace_slug).await?;

    println!(
        "{} '{}'",
        "Removed workspace".green(),
        workspace_id.to_string().bold()
    );

    Ok(())
}
