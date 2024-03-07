use crate::app::commands;
use crate::app::error::AppError::WorkspaceIdError;
use crate::app::error::Result;

pub async fn workspace_list() {
    println!("Listing all workspaces");
    let client = commands::anythingllm_client();
    let workspaces = client.workspace_list().await.unwrap();
    for ws in workspaces {
        println!("{:>2}: {}", ws.id, ws.name);
    }
}

pub async fn workspace_create(workspace_name: &str) {
    println!("Adding workspace: {}", workspace_name);
    let client = commands::anythingllm_client();
    client.workspace_create(workspace_name).await.unwrap();
}

pub async fn workspace_delete(workspace_id: u8) -> Result<()> {
    let client = commands::anythingllm_client();
    let workspace_slug = client
        .workspace_list()
        .await?
        .iter()
        .find(|ws| ws.id == workspace_id)
        .ok_or(WorkspaceIdError(workspace_id))?
        .slug
        .clone();

    client.workspace_delete(&workspace_slug).await?;

    println!("Removed workspace: {}", workspace_id);

    Ok(())
}
