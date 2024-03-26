//! aza
//!
//!
//!
use std::path::PathBuf;

use ayda::anythingllm::ChatMode;
use clap::Parser;
use color_eyre::owo_colors::OwoColorize;
use eyre::Context;
use tokio::select;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use ayda::app::{commands::admin, commands::workspace, commands::zotero, Commands::*};
use ayda::app::{Cli, SourceType, ZoteroCmd};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let (non_blocking_stdio, _guard) = tracing_appender::non_blocking(std::io::stdout());

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::ERROR)
        .with_writer(non_blocking_stdio)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config_path = ayda::Config::check_config()?;
    color_eyre::install()?;

    let cli = Cli::parse();

    select! {
        _ = command(config_path, cli) => {},
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("detected interrupt, exiting");
        },
    }

    Ok(())
}

#[tracing::instrument(skip(config_path, cli))]
async fn command(config_path: PathBuf, cli: Cli) -> eyre::Result<()> {
    match cli.command {
        Create { workspace_name } => workspace::create::create(workspace_name)
            .await
            .wrap_err("unable to create workspace"),

        List {} => workspace::list().await.wrap_err("unable to list workspace"),

        Delete {
            workspace_name: Some(name),
            all: false,
        } => {
            // Delete a specific workspace
            match workspace::delete(name).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{}", e.to_string().red());
                    Err(e)
                }
            }
        }

        Delete {
            workspace_name: None,
            all: true,
        } => {
            // Delete all workspaces
            match workspace::delete_all().await {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{}", e.to_string().red());
                    Err(e)
                }
            }
        }

        Import {
            source_type,
            source_name,
        } => match source_type {
            SourceType::Zotero {} => workspace::import_zotero(source_name)
                .await
                .wrap_err("unable to import zotero collection"),
            SourceType::Folder {} => workspace::import_folder(PathBuf::from(source_name))
                .await
                .wrap_err("unable to import file"),
            SourceType::Item {} => workspace::import_item()
                .await
                .wrap_err("unable to import item"),
        },

        Chat { workspace_name } => workspace::chat(workspace_name, ChatMode::Chat)
            .await
            .wrap_err("unable to chat with workspace"),

        Query { workspace_name } => workspace::chat(workspace_name, ChatMode::Query)
            .await
            .wrap_err("unable to query workspace"),

        Zotero {
            command: ZoteroCmd::ListCollections,
        } => zotero::list_collections().await.wrap_err("unable to list Zotero collections"),

        Zotero {
            command: ZoteroCmd::Enhance { collection_name },
        } => zotero::enhance_collection(collection_name)
            .await
            .wrap_err("unable to enhance collection"),

        Config {} => admin::configure(&config_path)
            .wrap_err("unable to configure application"),

        _ => {
            println!(
                "Invalid combination of arguments. Please consult the documentation for available commands"
            );
            Ok(())
        }
    }
    .wrap_err("couldn't execute command")
}
