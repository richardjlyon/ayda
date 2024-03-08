//! Zotero commands
//!
use std::env;
use std::io::{self, Write};

use colored::*;
use convert_case::{Case, Casing};
use dotenv::dotenv;

use crate::anythingllm::api::workspaces::UpdateParameter;
use crate::app::commands;
use crate::zotero::error::ZoteroError;
use crate::zotero::error::ZoteroError::*;

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

fn get_user_input(prompt: &str, max_value: usize) -> Result<u8, ZoteroError> {
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

/// Create an AnythingLLM workspace and add a Zotero collection to it
pub async fn zotero_add() -> Result<(), ZoteroError> {
    dotenv().ok();
    let zotero_library_root_path = &env::var("ZOTERO_LIBRARY_ROOT_PATH")?;
    let zotero = commands::zotero_client();
    let anythingllm = commands::anythingllm_client();

    let mut stdout = io::stdout();

    println!("{}", "Adding a zotero collection".green().bold());

    // Get collection to add

    let collections = zotero.zotero_list().await?;
    println!("{}", "Available collections:".green());
    for (index, c) in collections.clone().iter().enumerate() {
        println!("[{:>2}] {}", index + 1, c.name);
    }

    let collection_id = get_user_input("Enter the collection number to add: ", collections.len())?;
    let selected_collection = collections.get((collection_id - 1) as usize).unwrap();

    let pdfs = zotero.pdf_items(&selected_collection.key).await?;
    if pdfs.is_empty() {
        println!("No PDFs found in collection");
        return Ok(());
    }

    // Get workspace to add to or create new workspace

    let workspaces = anythingllm.workspace_list().await?;

    println!("{}", "Available workspaces:".green());
    for (index, w) in workspaces.iter().enumerate() {
        println!("[{:>2}] {} ({})", index + 1, w.name, w.slug);
    }

    print!(
        "{}",
        "Enter the workspace number to add to or 0 to create a new workspace: ".green()
    );

    stdout.flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let workspace_id: u8 = input.trim().parse().expect("Please enter a number");
    if workspace_id > workspaces.len() as u8 {
        return Err(InvalidWorkspaceId(collection_id));
    }

    dbg!(&workspace_id);

    let selected_workspace = match workspace_id {
        0 => {
            let suggested_name =
                format!("Zotero {}", selected_collection.name.to_case(Case::Title));
            print!(
                "{}",
                format!(
                    "Enter the name of the new workspace or 'return' to create workspace '{}': ",
                    suggested_name.bold()
                )
                .to_string()
                .green()
            );
            stdout.flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let workspace_name = input.trim();

            match workspace_name {
                "" => anythingllm.workspace_create(&suggested_name).await?,
                _ => anythingllm.workspace_create(&workspace_name).await?,
            }
        }
        _ => (*workspaces.get((workspace_id - 1) as usize).unwrap()).clone(),
    };

    dbg!(&selected_workspace);

    println!(
        "{}",
        format!(
            "Adding to workspace: {}",
            selected_workspace.name.to_string().green().bold()
        )
        .green()
    );

    println!("Selected workspace: {:?}", selected_workspace);

    // Add collection to workspace

    anythingllm
        .workspace_create(&selected_collection.name)
        .await?;

    for pdf in pdfs.iter().take(3) {
        // FIXME: Remove take(1)

        let document_filepath = pdf.filepath(zotero_library_root_path).expect("No filepath");
        let document = anythingllm
            .document_add(&document_filepath)
            .await
            .expect("No document");

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
