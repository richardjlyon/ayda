use crate::anythingllm::workspace::models::Workspace;
use crate::anythingllm::{ChatMode, Document};
use crate::app::commands;
use crate::app::commands::workspace::import::UpdateParameter;
use crate::zotero::item::model::{Creator, Item, ItemUpdateData, Tag};
use crate::Config;
use dialoguer::Confirm;
use futures::StreamExt;
use std::path::PathBuf;
use tracing::{info, instrument};
use uuid::Uuid;

/// Enhance a collection of PDFs.
///
/// This function iterates through every PDF item in the collection. For each item, it fetches the
/// item data from the Zotero API, creates a workspace for it and embeds it in the workspace. It
/// interrogates the item to obtain its title and author, and generates an abstract and keywords.
/// It updates the item metadata in the Zotero database, marks it with a tag to mark it as processed,
/// and deletes the custom workspace.
///
/// NOTE: This function alters a Zotero database and is not reversible. Use at own discretion.
///
#[instrument]
pub async fn enhance_collection(collection_name: String) -> eyre::Result<()> {
    // confirm deletion

    let confirm = Confirm::new()
        .with_prompt(format!(
            "This will modify Zotero collection '{}' and cannot be undone. Are you sure you wish to proceed?",
            collection_name
        ))
        .interact()?;

    if !confirm {
        println!("Cancelled");
        return Ok(());
    }

    // get collection

    let zotero = commands::zotero_client();
    let collection = zotero.collection_from_name(&collection_name).await;
    let collection = match collection {
        Ok(c) => {
            println!("Collection OK");
            c
            // sp.finish_ok("Collection OK".to_string());
        }
        Err(_) => {
            println!("Collection '{}' doesn't exist", collection_name);
            // sp.finish_error(format!("Collection '{}' doesn't exist", collection_name));
            return Ok(());
        }
    };

    // get pdfs

    let items: Vec<Item> = zotero
        .get_collections_collection_key_items_batched(collection.key)
        .collect()
        .await;

    let pdfs: Vec<Item> = items.into_iter().filter(|item| item.is_pdf()).collect();

    if !pdfs.is_empty() {
        // sp.finish_ok(format!("{} PDFS found", pdfs.len()));
        println!("{} PDFS found", pdfs.len());
    } else {
        // sp.finish_error("No PDFS found in collection".to_string());
        println!("No PDFS found in collection");
        return Ok(());
    }

    // process the pdfs

    for pdf in pdfs.iter() {
        println!("Processing {}", pdf.title);
        let metadata = get_metadata(pdf.clone()).await?;

        let updated_item = zotero.change_parent_item(&pdf, &metadata).await;

        if updated_item.is_ok() {
            println!("{} updated", pdf.title);
        } else {
            println!("{} not updated", pdf.title);
        }
    }

    Ok(())
}

/// Enhance a PDF item.
async fn get_metadata(pdf: Item) -> eyre::Result<ItemUpdateData> {
    let anythingllm = commands::anythingllm_client();
    let workspace_name = format!("workspace_{}", Uuid::new_v4());
    let workspace = anythingllm.create_workspace(&workspace_name).await?;

    let config = Config::from_file()?;
    let zotero_library_root_path = PathBuf::from(config.zotero_library_root_path);
    let document_filepath = match pdf
        .filepath(&zotero_library_root_path)
        .ok_or(eyre::eyre!("No file path"))
    {
        Ok(p) => p,
        Err(_) => {
            anythingllm.delete_workspace_slug(&workspace.slug).await?;
            return Err(eyre::eyre!("No file path"));
        }
    };

    let doc = match anythingllm.post_document_upload(&document_filepath).await {
        Ok(d) => d,
        Err(_) => {
            anythingllm.delete_workspace_slug(&workspace.slug).await?;
            return Err(eyre::eyre!("Document upload failed"));
        }
    };

    let _ = match anythingllm
        .update_embeddings(
            &workspace.slug,
            vec![doc.clone().location.unwrap()],
            UpdateParameter::Adds,
        )
        .await
    {
        Ok(_) => (),
        Err(_) => {
            anythingllm.delete_workspace_slug(&workspace.slug).await?;
            return Err(eyre::eyre!("Embedding failed"));
        }
    };

    let update_data = interrogate_doc(&workspace, &doc).await;

    anythingllm.delete_workspace_slug(&workspace.slug).await?;

    Ok(update_data)
}

async fn interrogate_doc(workspace: &Workspace, doc: &Document) -> ItemUpdateData {
    let anythingllm = commands::anythingllm_client();

    let doc_title = doc.clone().title.unwrap();
    let abstract_query = format!(
        "Summarise '{}' in 300 words. Omit the title, author, and line breaks.",
        &doc_title
    );
    let keywords_query = format!("[INST]This is an academic article[/INST] Generate 3 keywords for {}. Display the result as a comma separated list", &doc_title);

    let mut doc_abstract = anythingllm
        .post_workspace_slug_chat(&workspace.slug, &abstract_query, &ChatMode::Chat)
        .await
        .unwrap();

    // remove \n from abstract
    let doc_abstract = doc_abstract.text_response.replace("\n", "");

    let keywords = anythingllm
        .post_workspace_slug_chat(&workspace.slug, &keywords_query, &ChatMode::Chat)
        .await
        .unwrap();

    let keywords: Vec<String> = keywords
        .text_response
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let mut tags: Vec<Tag> = keywords
        .iter()
        .take(3)
        .map(|tag| Tag { tag: tag.clone() })
        .collect();

    tags.push(Tag {
        tag: "ayda".to_string(),
    });

    ItemUpdateData {
        abstract_note: Some(doc_abstract),
        tags: Some(tags),
        ..Default::default()
    }
}
