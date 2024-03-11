//! Zotero commands
//!
use std::env;
use std::io::{self, Write};

use colored::*;
use convert_case::{Case, Casing};
use dotenv::dotenv;

use crate::anythingllm::api::workspaces::UpdateParameter;
use crate::anythingllm::error::LLMError::{
    DocumentAddError, DocumentExistsError, DocumentNotFoundError, NoDocumentsError,
    ServerResponseFail,
};
use crate::anythingllm::models::workspace::Workspace;
use crate::app::commands;
use crate::zotero::error::ZoteroError;
use crate::zotero::error::ZoteroError::*;
use crate::zotero::models::collection::CollectionResponseData;

/// List Zotero collections
pub async fn zotero_list() -> Result<(), ZoteroError> {
    println!("{}", "Listing all zotero collections".green());

    let client = commands::zotero_client();
    let collections = client.zotero_list().await;

    for (index, c) in collections.unwrap().iter().enumerate() {
        println!("{:>2}: {}", index, c.name);
    }

    Ok(())
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

/// Create an AnythingLLM workspace and add a Zotero collection to it
pub async fn zotero_add() -> Result<(), ZoteroError> {
    dotenv().ok();
    let zotero_library_root_path = &env::var("ZOTERO_LIBRARY_ROOT_PATH")?;
    let zotero = commands::zotero_client();
    let anythingllm = commands::anythingllm_client();

    println!("{}", "Adding a zotero collection".green().bold());

    // Get collection to add to

    let collections = zotero.zotero_list().await?;
    list_collections(&collections);
    let collection_id = get_user_int("Enter the collection number to add: ", collections.len())?;
    let selected_collection = collections.get((collection_id - 1) as usize).unwrap();

    let pdfs = zotero.pdf_items(&selected_collection.key).await?;
    if pdfs.is_empty() {
        println!("No PDFs found in collection");
        return Ok(());
    }

    // Get workspace to add to

    let workspaces = anythingllm.workspace_list().await?;
    list_workspaces(&workspaces);
    let workspace_id: u8 = get_user_int(
        "Enter the workspace number or 0 to create a new workspace: ",
        workspaces.len(),
    )?;

    let selected_workspace = match workspace_id {
        0 => {
            let suggested_name =
                format!("Zotero {}", selected_collection.name.to_case(Case::Title));
            let workspace_name = get_user_string(&format!(
                "Enter the name of the new workspace or 'Return'' to create workspace '{}': ",
                suggested_name
            ))?;
            match workspace_name.as_str() {
                "" => anythingllm.workspace_create(&suggested_name).await?,
                _ => anythingllm.workspace_create(&workspace_name).await?,
            }
        }
        _ => (*workspaces.get((workspace_id - 1) as usize).unwrap()).clone(),
    };

    println!(
        "Adding to workspace: {}",
        selected_workspace.name.to_string().green().bold()
    );

    for pdf in pdfs.iter() {
        let document_filepath = pdf.filepath(zotero_library_root_path).unwrap();

        let document = match anythingllm.document_add(&document_filepath).await {
            Ok(doc) => doc,
            Err(DocumentExistsError(m)) => {
                println!("Document exists: {}", m);
                continue;
            }
            Err(DocumentNotFoundError(m)) => {
                println!("Document not found: {}", m);
                continue;
            }
            Err(DocumentAddError(m)) => {
                println!("Document add error: {}", m);
                continue;
            }
            Err(ServerResponseFail(m)) => {
                println!("Server response failure: {}", m);
                continue;
            }
            Err(NoDocumentsError(m)) => {
                println!("Server response failure: {}", m);
                continue;
            }
            Err(e) => {
                println!("Other document add error: {} {}", e, document_filepath);
                continue;
            }
        };

        anythingllm
            .workspace_update_embeddings(
                &selected_workspace.slug,
                vec![&document.doc_filepath_internal()],
                UpdateParameter::Adds,
            )
            .await?;
    }

    Ok(())
}

fn list_collections(collections: &[CollectionResponseData]) {
    for (index, c) in collections.iter().enumerate() {
        println!("[{:>2}] {}", index + 1, c.name);
    }
}

fn list_workspaces(workspaces: &[Workspace]) {
    for (index, w) in workspaces.iter().enumerate() {
        println!("[{:>2}] {} ({})", index + 1, w.name, w.slug);
    }
}
