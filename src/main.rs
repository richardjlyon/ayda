use clap::Parser;
use tokio;
use zot2llm::app::commands::document::document_add;
use zot2llm::app::commands::workspace::{workspace_create, workspace_delete};
use zot2llm::app::commands::*;
use zot2llm::app::*;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Workspace { command } => match command {
            WorkspaceCommands::List => {
                workspace_list().await;
            }
            WorkspaceCommands::Create { workspace_name } => {
                workspace_create(&workspace_name).await;
            }
            WorkspaceCommands::Delete { workspace_id } => {
                if let Err(e) = workspace_delete(workspace_id).await {
                    eprintln!("Error: {}", e);
                }
            }
        },

        Commands::Document { command } => match command {
            DocumentCommands::Add {
                document_filepath,
                workspace_id,
            } => {
                if let Err(e) = document_add(&document_filepath, workspace_id).await {
                    eprintln!("Error: {}", e);
                }
            }
            DocumentCommands::Remove { document_name } => {
                println!("Removing document: {}", document_name);
                // Implement the logic to remove a document here.
            }
        },
    }
}
