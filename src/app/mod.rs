//! A command line application.

use clap::{Parser, Subcommand, ValueEnum};

pub mod commands;

#[derive(Parser)]
#[clap(
    name = "ada",
    about = "A tool for interrogating your documents with a large language model."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new workspace
    Create {
        /// Name of the workspace to create
        workspace_name: String,
    },
    /// List all workspaces
    List {},

    /// Delete a workspace
    Delete {
        /// Name of the workspace to delete
        #[clap(required_unless_present = "all")]
        workspace_name: Option<String>,

        /// Force delete all workspaces
        #[clap(long)]
        all: bool,
    },

    /// Import items into a workspace from a specific source
    Import {
        /// The type of source to import
        #[clap(value_enum, long)]
        source: SourceType,

        /// The source to import from
        source_name: String,
    },

    /// Chat with a workspace
    Chat {
        /// Name of the workspace to chat with
        workspace_name: String,
    },

    /// Query a workspace
    Query {
        /// Name of the workspace to query
        workspace_name: String,
    },

    /// Manage Zotero collections
    Zotero {
        #[clap(subcommand)]
        command: ZoteroCmd,
    },

    /// Configure the application
    Config {},
}

#[derive(Subcommand)]
pub enum ZoteroCmd {
    /// List all Zotero collections
    ListCollections,

    /// Enhance a collection
    Enhance {
        /// The name of the collection to enhance
        collection_name: String,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum SourceType {
    /// Import from Zotero
    Zotero,
    /// Import from a folder
    Folder,
    /// Import a single item
    Item,
}

// Utility to create a table from headers and data
// e.g.
//
// ID  NAME                         SLUG
// 468 first workspace              first-workspace
// 472 workspace with a long title  workspace-with-a-long-title
//
fn display_table(column_titles: Vec<&str>, data: Vec<Vec<String>>) {
    let header_style = console::Style::new().bold();
    let data_style = console::Style::new();

    // find the longest string in each column
    let mut column_widths = [0, 0, 0];
    for row in data.iter() {
        for (i, cell) in row.iter().enumerate() {
            if cell.len() > column_widths[i] {
                column_widths[i] = cell.len();
            }
        }
    }

    println!();
    // print the headers in fields defined by column_widths
    for (i, column_title) in column_titles.iter().enumerate() {
        let padded_header = format!("{:<width$}", column_title, width = column_widths[i]);
        print!("{} ", header_style.apply_to(padded_header));
    }
    println!();

    for row in data.iter() {
        for (i, cell) in row.iter().enumerate() {
            let padded_cell = format!("{:<width$}", cell, width = column_widths[i]);
            print!("{} ", data_style.apply_to(padded_cell));
        }
        println!();
    }
}
