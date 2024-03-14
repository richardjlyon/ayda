//! A command line application.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod commands;

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
        command: WorkspaceCmd,
    },

    /// Manage documents
    Document {
        #[clap(subcommand)]
        command: DocumentCmd,
    },

    /// Manage Zotero collections
    Zotero {
        #[clap(subcommand)]
        command: ZoteroCmd,
    },
}

#[derive(Subcommand)]
pub enum WorkspaceCmd {
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
        workspace_id: i32,
    },
}

#[derive(Subcommand)]
pub enum DocumentCmd {
    /// List all documents
    List,

    /// Add a new document
    Upload {
        /// File path of the document
        document_filepath: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum ZoteroCmd {
    /// List all collections
    List,

    /// Add a new collection
    Add,
}
