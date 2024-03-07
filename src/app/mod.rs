pub mod commands;
pub mod error;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "zot2llm",
    about = "A tool for managing workspaces and documents in zot2llm."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage workspaces
    Workspace {
        #[clap(subcommand)]
        command: WorkspaceCommands,
    },
    /// Manage documents
    Document {
        #[clap(subcommand)]
        command: DocumentCommands,
    },
}

#[derive(Subcommand)]
pub enum WorkspaceCommands {
    /// List all workspaces
    List,
    /// Create a new workspace
    Create {
        /// Name of the workspace to create
        workspace_name: String,
    },
    /// Delete an existing workspace
    Delete {
        /// id of the workspace to delete (use 'list' to get the id)
        workspace_id: u8,
    },
}

#[derive(Subcommand)]
pub enum DocumentCommands {
    /// Adds a new document
    Add {
        /// Name of the document to add
        document_filepath: String,
        /// Workspace to add the document to
        workspace_id: u8,
    },
    /// Removes an existing document
    Remove {
        /// Name of the document to remove
        document_name: String,
    },
}
