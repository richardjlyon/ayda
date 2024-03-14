use std::{env, io};
use std::io::Write;
use std::path::PathBuf;

use colored::*;
use convert_case::{Case, Casing};
use dotenv::dotenv;

use crate::anythingllm::workspace::Workspace;
use crate::app::commands;
use crate::zotero::api::models::collection::Collection;
use crate::zotero::api::models::item::Item;
use crate::zotero::client::ZoteroClient;
use crate::zotero::error::ZoteroError;
use crate::zotero::error::ZoteroError::*;

// use crate::anythingllm::api::workspaces::UpdateParameter;

/// List all Zotero collections
///
pub async fn zotero_list() -> Result<(), ZoteroError> {
    println!("{}", "Listing all zotero collections".green());

    let client = commands::zotero_client();
    let collections = client.get_collections(None).await;

    for (index, c) in collections.unwrap().iter().enumerate() {
        println!("{:>2}: {}", index, c.name);
    }

    Ok(())
}

/// Create an AnythingLLM workspace and add a Zotero collection to it
pub async fn zotero_add() -> Result<(), ZoteroError> {
    println!("{}", "Adding a zotero collection".green().bold());

    let zotero = commands::zotero_client();

    let selected_collection = get_collection(&zotero).await?;
    let pdfs = get_pdfs(zotero, &selected_collection).await;
    if pdfs.is_empty() {
        println!("No PDFs found in collection");
        return Ok(());
    }

    let selected_workspace = get_workspace(selected_collection).await?;
    println!(
        "Adding to workspace: {}",
        selected_workspace.name.to_string().green().bold()
    );

    dotenv().ok();
    let zotero_library_root_path = &env::var("ZOTERO_LIBRARY_ROOT_PATH")?;
    let zotero_library_root_path = PathBuf::from(zotero_library_root_path);
    let anythingllm = commands::anythingllm_client();
    for pdf in pdfs.iter() {
        let document_filepath = pdf.filepath(&zotero_library_root_path).unwrap();

        let document = match anythingllm.post_document_upload(&document_filepath).await {
            Ok(doc) => doc,
            Err(e) => {
                println!("Other document add error: {} {}", e, document_filepath.to_string_lossy());
                continue;
            }
        };

        // anythingllm
        //     .workspace_update_embeddings(
        //         &selected_workspace.slug,
        //         vec![&document.doc_filepath_internal()],
        //         UpdateParameter::Adds,
        //     )
        //     .await?;
    }

    Ok(())
}

async fn get_workspace(selected_collection: Collection) -> Result<Workspace, ZoteroError> {
    let anythingllm = commands::anythingllm_client();
    let workspaces = anythingllm.get_workspaces().await?;
    list_workspaces(&workspaces);
    let workspace_id: u8 = get_user_int(
        "Enter the workspace number or 0 to create a new workspace: ",
        workspaces.len(),
    )?;

    let selected_workspace = if workspace_id == 0 {
        let workspace_name = get_user_string(&format!(
            "Enter the name of the new workspace or 'Return'' to create workspace 'Zotero {}': ",
            selected_collection.name.to_case(Case::Title)
        ))?;
        anythingllm.post_workspace_new(&workspace_name).await?
    } else {
        workspaces.get((workspace_id - 1) as usize).unwrap().clone()
    };

    Ok(selected_workspace)
}

// Get the Zotero collection to add to the workspace
async fn get_collection(zotero: &ZoteroClient) -> Result<Collection, ZoteroError> {
    let collections = zotero.get_collections(None).await?;
    list_collections(&collections);
    let collection_id = get_user_int("Enter the collection number to add: ", collections.len())?;
    let selected_collection = collections
        .get((collection_id - 1) as usize)
        .unwrap()
        .clone();

    Ok(selected_collection)
}

async fn get_pdfs(zotero: ZoteroClient, selected_collection: &Collection) -> Vec<Item> {
    let params = Some(
        [
            ("itemType", "attachment"),
            ("format", "json"),
            ("linkMode", "imported_file"),
            ("limit", "100"),
        ]
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect(),
    );

    let items = zotero
        .get_collections_collection_key_items(&selected_collection.key, params)
        .await
        .unwrap();

    items
        .iter()
        .filter(|i| i.content_type == Some("application/pdf".to_string()))
        .cloned()
        .collect()
}

pub enum UpdateParameter {
    Adds,
    Deletes,
}

fn get_user_int(prompt: &str, max_value: usize) -> Result<u8, ZoteroError> {
    let mut stdout = io::stdout();
    print!("{}", prompt.green());
    stdout.flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let value: u8 = input.trim().parse().expect("Please enter a number");
    if value > max_value as u8 {
        return Err(InvalidInput(value));
    }
    Ok(value)
}

fn get_user_string(prompt: &str) -> Result<String, ZoteroError> {
    let mut stdout = io::stdout();
    print!("{}", prompt.green());
    stdout.flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let value = input.trim().to_string();
    Ok(value)
}

fn list_collections(collections: &[Collection]) {
    for (index, c) in collections.iter().enumerate() {
        println!("[{:>2}] {}", index + 1, c.name);
    }
}

fn list_workspaces(workspaces: &[Workspace]) {
    for (index, w) in workspaces.iter().enumerate() {
        println!("[{:>2}] {} ({})", index + 1, w.name, w.slug);
    }
}
