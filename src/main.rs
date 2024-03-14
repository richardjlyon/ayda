//! zot2llm
//!
//!
//!
use clap::Parser;
use eyre::Context;

use zot2llm::app::commands::document::*;
use zot2llm::app::commands::workspace::*;
use zot2llm::app::commands::zotero::{zotero_add, zotero_list};
use zot2llm::app::*;

// #[derive(Deserialize, Debug)]
// struct Config {
//     zotero_user_id: String,
//     anythingllm_api_key: String,
// }

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    // let dirs = directories_next::ProjectDirs::from("com", "richard", "zot2llm").unwrap();
    // let config_path = dirs.config_dir().join("config.json");
    // let file = std::fs::File::open(config_path).wrap_err("could not open config file")?;
    // let data: Config = serde_json::from_reader(file).wrap_err("config file invalid")?;
    // println!("{:?}", data);

    use Commands::*;
    match cli.command {
        Workspace {
            command: WorkspaceCmd::List,
        } => workspace_list().await.wrap_err("unable to list workspace"),

        Workspace {
            command: WorkspaceCmd::Create { workspace_name },
        } => workspace_create(&workspace_name)
            .await
            .wrap_err("unable to create workspace"),

        Workspace {
            command: WorkspaceCmd::Delete { workspace_id },
        } => workspace_delete(workspace_id)
            .await
            .wrap_err("workspace_delete"),

        Document {
            command: DocumentCmd::List,
        } => document_list().await.wrap_err("document_list"),

        Document {
            command: DocumentCmd::Upload { document_filepath },
        } => document_upload(&document_filepath)
            .await
            .wrap_err("document_add"),

        Zotero {
            command: ZoteroCmd::List,
        } => zotero_list().await.wrap_err("zotero_list"),

        Zotero {
            command: ZoteroCmd::Add,
        } => zotero_add().await.wrap_err("zotero_add"),
    }
    .wrap_err("couldn't execute command")
}
