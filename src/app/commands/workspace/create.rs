use colored::*;
use eyre::Context;

use crate::anythingllm::error::LLMError;
use crate::app::commands;

/// Create a new workspace.
/// Method will fail if a workspace with the same name already exists.
pub async fn create(workspace_name: String) -> eyre::Result<()> {
    let client = commands::anythingllm_client();

    // fail if workspace(s) already exists

    match client.get_workspace_by_name(&workspace_name).await {
        Ok(_) => {
            println!(
                "{}",
                format!("Workspace '{}' already exists. Exiting...", workspace_name).red()
            );
            return Ok(());
        }
        Err(LLMError::MultipleWorkspacesError { .. }) => {
            println!(
                "{}",
                format!(
                    "Multiple workspaces found with the name '{}'. Exiting...",
                    workspace_name
                )
                .red()
            );
            return Ok(());
        }
        Err(_) => {}
    }

    // create the workspace

    client
        .create_workspace(&workspace_name)
        .await
        .wrap_err("couldn't create new workspace")?;

    println!("Created workspace '{}'", workspace_name);

    Ok(())
}
