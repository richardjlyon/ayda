use colored::*;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;

use futures::{stream, StreamExt};

use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};

use crate::zotero::collection::models::Collection;
use tracing::{event, info, span, Instrument, Level};

use crate::anythingllm::workspace::models::Workspace;
use crate::app::commands;
use crate::zotero::item::models::Item;
use crate::Config;
use eyre::eyre;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Import items from a Zotero collection to a workspace.
///
pub async fn import_zotero(collection_name: String) -> eyre::Result<()> {
    println!("Importing documents from '{}'", collection_name);

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

    let pdf_file_paths = file_paths(pdfs);

    let workspace = match create_or_get_workspace("zotero", &collection_name).await {
        Ok(workspace) => workspace,
        Err(e) => {
            return Err(e);
        }
    };

    let uploaded_pdf_ids = match upload_pdfs(pdf_file_paths).await {
        Ok(uploaded_pdf_ids) => uploaded_pdf_ids,
        Err(e) => {
            return Err(e);
        }
    };

    if embed_pdfs(&workspace, uploaded_pdf_ids).await.is_err() {
        return Err(eyre!("Error embedding PDFs"));
    }

    println!("done");

    Ok(())
}

/// Import items from a folder into a workspace.
///
pub async fn import_folder(folder_path: PathBuf) -> eyre::Result<()> {
    println!("Importing documents from {}", folder_path.display());

    let pdfs = match get_pdf_filepaths_from_directory(&folder_path).await {
        Ok(pdfs) => pdfs,
        Err(e) => {
            return Err(e);
        }
    };

    let folder_name = folder_path.file_name().unwrap().to_string_lossy();
    let workspace = match create_or_get_workspace("folder", &folder_name).await {
        Ok(workspace) => workspace,
        Err(e) => {
            return Err(e);
        }
    };

    let uploaded_pdf_ids = match upload_pdfs(pdfs).await {
        Ok(uploaded_pdf_ids) => uploaded_pdf_ids,
        Err(e) => {
            return Err(e);
        }
    };

    if embed_pdfs(&workspace, uploaded_pdf_ids).await.is_err() {
        return Err(eyre!("Error embedding PDFs"));
    }

    println!("done");

    Ok(())
}

/// Import an item.

pub async fn import_item() -> eyre::Result<()> {
    Ok(())
}

// A spinner that can be updated with a message
#[allow(dead_code)]
struct Spinner {
    message: String,
    spinner: ProgressBar,
}

impl Spinner {
    fn new(message: String) -> Spinner {
        let spinner_style = ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap();

        let spinner = ProgressBar::new_spinner()
            // .with_prefix(prefix.clone())
            .with_message(message.clone())
            .with_style(spinner_style.clone());
        spinner.enable_steady_tick(Duration::from_millis(100));

        Spinner {
            message: message.to_string(),
            spinner,
        }
    }

    fn finish_ok(&self, message: String) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg:.green}")
                .unwrap(),
        );
        self.spinner.finish_with_message(message);
    }

    fn finish_error(&self, message: String) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg:.red}")
                .unwrap(),
        );
        self.spinner.finish_with_message(message);
    }
}

pub enum UpdateParameter {
    Adds,
    Deletes,
}

pub async fn get_collection(collection_name: &str) -> eyre::Result<Collection> {
    let sp = Spinner::new("Checking collection...".to_string());
    let zotero = commands::zotero_client();
    let collection = zotero.collection_from_name(&collection_name).await;

    match collection {
        Ok(collection) => {
            sp.finish_ok("Collection OK".to_string());
            Ok(collection)
        }
        Err(_) => {
            sp.finish_error("Collection not found. Run 'zotero list-collections'".to_string());
            Err(eyre!("Error checking collection"))
        }
    }
}

pub async fn get_pdfs_from_collection(collection: &Collection) -> eyre::Result<Vec<Item>> {
    let sp = Spinner::new("Checking PDFs...".to_string());
    let zotero = commands::zotero_client();
    let items: Vec<Item> = zotero
        .get_collections_collection_key_items_batched(collection.clone().key)
        .collect()
        .await;
    let pdfs: Vec<Item> = items.into_iter().filter(|item| item.is_pdf()).collect();

    if !pdfs.is_empty() {
        sp.finish_ok(format!("{} PDFS found", pdfs.len()));
        Ok(pdfs)
    } else {
        sp.finish_error("No PDFS found in collection".to_string());
        Err(eyre!("Error getting PDFs"))
    }
}

async fn get_pdf_filepaths_from_directory(folder_path: &PathBuf) -> eyre::Result<Vec<PathBuf>> {
    let sp = Spinner::new("Getting PDFs...".to_string());
    match folder_path.exists() {
        true => {
            let mut pdf_file_paths = Vec::new();
            let mut dir = fs::read_dir(folder_path).await?;
            while let Some(entry) = dir.next_entry().await? {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == "pdf" {
                    pdf_file_paths.push(path);
                }
            }
            if !pdf_file_paths.is_empty() {
                sp.finish_ok(format!("{} PDFs found", pdf_file_paths.len()));
                Ok(pdf_file_paths)
            } else {
                sp.finish_error("No PDFs found in folder".to_string());
                Err(eyre!("No PDFs found in folder"))
            }
        }
        false => {
            sp.finish_error("Folder does not exist".to_string());
            Err(eyre!("Folder does not exist"))
        }
    }
}

async fn create_or_get_workspace(prefix: &str, collection_name: &str) -> eyre::Result<Workspace> {
    let sp = Spinner::new("Checking workspace...".to_string());
    let anythingllm = commands::anythingllm_client();
    let workspace_name = format!("{}-{}", prefix, collection_name);

    match anythingllm.get_workspace_by_name(&workspace_name).await {
        Ok(workspace) => {
            sp.finish_error("Workspace exists".to_string());
            let confirmation = Confirm::new()
                .with_prompt("  Do you want to continue?")
                .interact()
                .expect("Error getting confirmation");

            if !confirmation {
                sp.finish_error("Cancelled".to_string());
                return Err(eyre!("Error creating workspace"));
            }

            let _ = anythingllm.delete_workspace_slug(&workspace.slug).await;
        }
        Err(_) => {}
    }

    match anythingllm.create_workspace(&workspace_name).await {
        Ok(workspace) => {
            sp.finish_ok(format!("Created workspace {}", workspace.name));
            Ok(workspace)
        }
        Err(_) => {
            sp.finish_error("Error creating workspace".to_string());
            Err(eyre!("Error creating workspace"))
        }
    }
}

async fn upload_pdfs(file_paths: Vec<PathBuf>) -> eyre::Result<Vec<String>> {
    let mut failures = Vec::<PathBuf>::new(); // Declare failures as mutable

    let doc_count = file_paths.len();
    let bar = ProgressBar::new(doc_count as u64);
    let bar_style = ProgressStyle::default_bar()
        .template("{bar:100.cyan/blue} {pos:>7}/{len:7} {msg} {eta}")
        .unwrap();
    bar.set_style(bar_style.progress_chars("##-"));

    // share one copy between multiple readers using reference counting
    let anythingllm = Arc::new(commands::anythingllm_client());
    let failed_docs = Arc::new(Mutex::new(Vec::<PathBuf>::new()));

    let docs: Vec<_> = stream::iter(file_paths)
        .map(|document_filepath| {
            let span = span!(Level::INFO, "process PDF");
            let anythingllm = anythingllm.clone();
            let failed_docs = failed_docs.clone();
            let bar = bar.clone();

            async move {
                event!(Level::INFO, "Uploading");

                // Upload PDFs
                match anythingllm.post_document_upload(&document_filepath).await {
                    Ok(doc) => {
                        event!(Level::INFO, "upload success");
                        bar.inc(1);
                        doc.location
                    }
                    Err(_) => {
                        let mut failed_docs = failed_docs.lock().await;
                        failed_docs.push(document_filepath.clone());
                        event!(
                            Level::INFO,
                            "upload fail: {}",
                            document_filepath.as_path().display(),
                        );
                        None
                    }
                }
            }
            .instrument(span)
        })
        .buffered(100)
        .filter_map(|f| async { f })
        .collect()
        .await;

    bar.finish();

    let failed_docs_mutex_guard = failed_docs.lock().await;
    failures.append(&mut failed_docs_mutex_guard.clone());

    if failures.is_empty() {
        println!("{}", "  All documents uploaded successfully.".green());
    } else {
        let dirs = directories_next::ProjectDirs::from("com", "richardlyon", "aza").unwrap();
        let filename = format!(
            "log_{}.txt",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        );
        let log_path = dirs.config_dir().join(filename);
        let mut log_file = std::fs::File::create(&log_path).unwrap();
        for doc in failures.iter() {
            writeln!(log_file, "{}", doc.display()).unwrap();
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

async fn embed_pdfs(workspace: &Workspace, pdfs: Vec<String>) -> eyre::Result<()> {
    let sp = Spinner::new("Embedding PDFs in workspace...".to_string());
    let anythingllm = commands::anythingllm_client();

    match anythingllm
        .update_embeddings(&workspace.slug, pdfs, UpdateParameter::Adds)
        .await
    {
        Ok(_) => {
            sp.finish_ok("Embedding OK".to_string());
            Ok(())
        }
        Err(_) => {
            sp.finish_error("Error embedding".to_string());
            Err(eyre!("Error embedding"))
        }
    }
}

pub fn file_paths(pdfs: Vec<Item>) -> Vec<PathBuf> {
    let config = Config::from_file().unwrap();
    let zotero_library_root_path = PathBuf::from(config.zotero_library_root_path);
    let file_paths = pdfs
        .iter()
        .map(|pdf| pdf.filepath(&zotero_library_root_path).unwrap())
        .collect::<Vec<PathBuf>>();
    file_paths
}
