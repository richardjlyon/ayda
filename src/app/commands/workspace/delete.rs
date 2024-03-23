use colored::*;
use dialoguer::Confirm;

use crate::app::commands;

/// Delete a workspace.
pub async fn delete(workspace_name: String) -> eyre::Result<()> {
    let client = commands::anythingllm_client();
    let workspace = client.get_workspace_by_name(&workspace_name).await?;

    // fail if unique workspace doesn't exist

    match client.get_workspace_by_name(&workspace_name).await {
        Ok(_) => {}
        _ => {
            println!(
                "{}",
                format!("Workspace '{}' not unique. Exiting...", workspace_name).red()
            );
            return Ok(());
        }
    };

    // confirm deletion

    let confirm = Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to delete workspace '{}'?",
            workspace_name
        ))
        .interact()?;

    if !confirm {
        println!("Cancelled");
        return Ok(());
    }

    // delete the workspace

    let result = client.delete_workspace_slug(&workspace.slug).await;

    match result {
        Ok(_) => {
            println!("Deleted workspace '{}'", workspace_name);
        }
        Err(_) => {
            println!("Error deleting workspace '{}'", workspace_name);
        }
    }
    Ok(())
}

/// Delete all workspaces.

pub async fn delete_all() -> eyre::Result<()> {
    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to delete all workspace?")
        .interact()?;

    if !confirm {
        println!("Cancelled");
        return Ok(());
    }

    let client = commands::anythingllm_client();
    let workspaces = client.get_workspaces().await?;
    for ws in workspaces {
        let result = client.delete_workspace_slug(&ws.slug).await;
        match result {
            Ok(_) => {
                println!("Deleted workspace '{}'", ws.name);
            }
            Err(_) => {
                println!("Error deleting workspace '{}'", ws.name);
            }
        }
    }

    Ok(())
}
