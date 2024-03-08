use clap::Parser;

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
                if let Err(e) = workspace_list().await {
                    eprintln!("Error: {}", e);
                }
            }
            WorkspaceCommands::Create { workspace_name } => {
                if let Err(e) = workspace_create(&workspace_name).await {
                    eprintln!("Error: {}", e);
                }
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
            DocumentCommands::Remove { document_id } => {
                println!("Removing document: {}", document_id);
                // Implement the logic to remove a document here.
            }
        },

        Commands::Zotero { command } => match command {
            ZoteroCommands::List => {
                println!("Listing zotero collections");
                // Implement the logic to list zotero collections here.
            }
            ZoteroCommands::Add { collection_id } => {
                println!("Adding zotero collection: {}", collection_id);
                // Implement the logic to add a zotero collection here.
            }
        },
    }
}
