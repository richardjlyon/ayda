use eyre::Context;

use crate::anythingllm::workspace::Workspace;
use crate::app;
use crate::app::commands;

/// List all workspaces.
pub async fn list() -> eyre::Result<()> {
    let client = commands::anythingllm_client();
    let workspaces = client
        .get_workspaces()
        .await
        .wrap_err("couldn't get workspaces")?;

    if workspaces.is_empty() {
        println!("No workspaces found");
        return Ok(());
    }

    let column_titles = vec!["WORKSPACE"];
    let data = data_from_workspaces(workspaces);
    app::display_table(column_titles, data);

    Ok(())
}

fn data_from_workspaces(workspaces: Vec<Workspace>) -> Vec<Vec<String>> {
    let data: Vec<Vec<String>> = workspaces.iter().map(|ws| vec![ws.name.clone()]).collect();
    data
}
