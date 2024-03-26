use crate::anythingllm::workspace::models::Workspace;
use crate::anythingllm::{ChatMode, Document};
use crate::app::commands;
use crate::app::commands::workspace::import::{
    file_paths, get_collection, get_pdfs_from_collection, UpdateParameter,
};
use crate::zotero::item::models::{Item, ItemUpdateData, Tag};
use crate::Config;
use colored::Colorize;
use dialoguer::Confirm;
use futures::{stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{event, info, instrument, span, Instrument, Level};
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

    let collection = match get_collection(&collection_name).await {
        Ok(collection) => collection,
        Err(e) => {
            return Err(e);
        }
    };

    let pdfs = match get_pdfs_from_collection(&collection).await {
        Ok(docs) => docs,
        Err(e) => {
            return Err(e);
        }
    };

    match enhance_pdfs(pdfs).await {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}

async fn enhance_pdfs(pdfs: Vec<Item>) -> eyre::Result<Vec<String>> {
    let mut failures = Vec::<Item>::new();

    let doc_count = pdfs.len();
    let bar = ProgressBar::new(doc_count as u64);
    let bar_style = ProgressStyle::default_bar()
        .template("{bar:100.cyan/blue} {pos:>7}/{len:7} {msg} {eta}")
        .unwrap();
    bar.set_style(bar_style.progress_chars("##-"));

    // share one copy between multiple readers using reference counting
    let zotero = Arc::new(commands::zotero_client());
    let failed_docs = Arc::new(Mutex::new(Vec::<Item>::new()));

    let docs: Vec<_> = stream::iter(pdfs)
        .map(|pdf| {
            let span = span!(Level::INFO, "enhance PDF");
            let zotero = zotero.clone();
            let failed_docs = failed_docs.clone();
            let bar = bar.clone();

            async move {
                event!(Level::INFO, "Getting metadata for {}", pdf.title);
                let metadata = match get_metadata(pdf.clone()).await {
                    Ok(m) => {
                        bar.inc(1);
                        event!(Level::INFO, "Got metadata: {:?}", m);
                        m
                    }
                    Err(_) => {
                        bar.inc(1);
                        let mut failed_docs = failed_docs.lock().await;
                        failed_docs.push(pdf.clone());
                        event!(Level::INFO, "metadata failed");
                        return None;
                    }
                };

                event!(Level::INFO, "Updating parent item for  {}", pdf.title);
                match zotero.change_parent_item(&pdf, &metadata).await {
                    Ok(_) => Some(pdf.title),
                    Err(_) => {
                        let mut failed_docs = failed_docs.lock().await;
                        failed_docs.push(pdf.clone());
                        event!(Level::INFO, "upload fail: {}", pdf.title,);
                        None
                    }
                }
            }
            .instrument(span)
        })
        .buffered(20)
        .filter_map(|f| async { f })
        .collect()
        .await;

    bar.finish();

    let failed_docs_mutex_guard = failed_docs.lock().await;
    failures.append(&mut failed_docs_mutex_guard.clone());

    if failures.is_empty() {
        println!("{}", "  All documents enhanced successfully.".green());
    } else {
        let dirs = directories_next::ProjectDirs::from("com", "richardlyon", "aza").unwrap();
        let filename = format!(
            "log_{}.txt",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        );
        let log_path = dirs.config_dir().join(filename);
        let mut log_file = std::fs::File::create(&log_path).unwrap();
        for doc in failures.iter() {
            writeln!(log_file, "{}", doc.title).unwrap();
        }
        let message = format!(
            "  {} document(s) failed to upload. See {} for details.",
            failures.len(),
            log_path.display()
        );
        println!("{}", message.red());
    }

    Ok(docs)
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

    match anythingllm
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

    info!(
        "Embedded document {} in workspace {}",
        pdf.title, workspace.slug
    );

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

    let doc_abstract = anythingllm
        .post_workspace_slug_chat(&workspace.slug, &abstract_query, &ChatMode::Chat)
        .await
        .unwrap();

    // remove \n from abstract
    let doc_abstract = doc_abstract.text_response.replace('\n', "");

    let keywords = anythingllm
        .post_workspace_slug_chat(&workspace.slug, &keywords_query, &ChatMode::Chat)
        .await
        .unwrap();

    let keywords: Vec<String> = keywords
        .text_response
        .split(',')
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
