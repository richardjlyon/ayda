use colored::*;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;

use futures::{stream, StreamExt};

use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};

use tracing::{event, info, span, Instrument, Level};

use crate::app::commands;
use crate::zotero::item::model::Item;
use crate::Config;

/// Import items from a Zotero collection to a workspace.
///
pub async fn import_zotero(collection_name: String) -> eyre::Result<()> {
    println!("Importing documents from {}", collection_name);

    // Verify that the collection exists

    let sp = Spinner::new("[1/6]".to_string(), "Checking collection...".to_string());

    let zotero = commands::zotero_client();
    let collection = zotero.collection_from_name(&collection_name).await;
    match collection {
        Ok(_) => {
            sp.finish_ok("Collection OK".to_string());
        }
        Err(_) => {
            sp.finish_error(format!("Collection '{}' doesn't exist", collection_name));
            return Ok(());
        }
    }

    // Get PDFs from the collection

    let sp = Spinner::new("[2/6]".to_string(), "Checking PDFs...".to_string());

    let items: Vec<Item> = zotero
        .get_collections_collection_key_items_batched(collection.unwrap().key)
        .collect()
        .await;

    let pdfs: Vec<Item> = items.into_iter().filter(|item| item.is_pdf()).collect();

    if !pdfs.is_empty() {
        sp.finish_ok(format!("{} PDFS found", pdfs.len()));
    } else {
        sp.finish_error("No PDFS found in collection".to_string());
        return Ok(());
    }

    // Upload the PDFs
    let config = Config::from_file().unwrap();
    let zotero_library_root_path = PathBuf::from(config.zotero_library_root_path);
    let file_paths = pdfs
        .iter()
        .map(|pdf| pdf.filepath(&zotero_library_root_path).unwrap())
        .collect::<Vec<PathBuf>>();

    let workspace_name = format!("zotero-{}", collection_name);
    let _ = upload_docs(&workspace_name, file_paths).await;

    println!("done");

    Ok(())
}

/// Import items from a folder into a workspace.
///
pub async fn import_folder(folder_path: PathBuf) -> eyre::Result<()> {
    println!("Importing documents from {}", folder_path.display());

    // Verify that the folder exists

    if !folder_path.exists() {
        println!("{}", "Folder does not exist.".red());
        return Ok(());
    }

    // Get the PDFs from the folder

    let pdf_files = get_pdfs_from_directory(&folder_path).await?;
    if pdf_files.is_empty() {
        println!("{}", "No files found in folder.".red());
        return Ok(());
    }

    // Upload the PDFs

    let folder_name = folder_path.file_name().unwrap().to_string_lossy();
    let workspace_name = format!("folder-{}", folder_name);
    let _ = upload_docs(&workspace_name, pdf_files).await;

    Ok(())
}

/// Import an item.

pub async fn import_item(folder_path: String) -> eyre::Result<()> {
    Ok(())
}

// Upload PDFs to the workspace
//
// This function will:
// - Create / replace workspace
// - Upload the PDFs to the server
// - Embed the PDFs in the workspace
//
// Upload failures are reported at the end of the process and logged to allow for manual
// intervention.
async fn upload_docs(workspace_name: &String, file_paths: Vec<PathBuf>) -> eyre::Result<()> {
    let anythingllm = commands::anythingllm_client();
    let workspace = anythingllm.get_workspace_by_name(&workspace_name).await;

    let sp = Spinner::new("[3/6]".to_string(), "Checking workspace...".to_string());

    if workspace.is_ok() {
        sp.finish_error("Workspace exists".to_string());

        let confirmation = Confirm::new()
            .with_prompt("  Workspace exists. Do you want to continue?")
            .interact()?;

        if !confirmation {
            println!("  Cancelled");
            return Ok(());
        }

        let _ = anythingllm
            .delete_workspace_slug(&workspace.unwrap().slug)
            .await;
    };

    let workspace = match anythingllm.create_workspace(&workspace_name).await {
        Ok(workspace) => {
            // sp.finish_ok("Workspace OK".to_string());
            workspace
        }
        Err(_) => {
            // sp.finish_error("Error creating workspace. Exiting".to_string());
            return Ok(());
        }
    };

    // [4/6] Upload PDFs

    let bar_style = ProgressStyle::default_bar()
        .template("{bar:100.cyan/blue} {pos:>7}/{len:7} {msg} {eta}")
        .unwrap();

    let doc_count = file_paths.len();
    let bar = ProgressBar::new(doc_count as u64);
    bar.set_style(bar_style.progress_chars("##-"));

    // share one copy between multiple readers using reference counting

    let anythingllm = std::sync::Arc::new(commands::anythingllm_client());
    let failed_docs = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::<PathBuf>::new()));

    let docs: Vec<_> = stream::iter(file_paths)
        .map(|document_filepath| {
            let span = span!(Level::INFO, "process PDF");
            let bar = bar.clone();
            let anythingllm = anythingllm.clone();
            let failed_docs = failed_docs.clone();
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

    // [5/6] Embed PDFs in workspace

    // FIXME correct PDF count for failures
    info!("embedding {} pdfs in workspace", doc_count);

    let sp = Spinner::new(
        "[5/6]".to_string(),
        "Embedding PDFs in workspace...".to_string(),
    );

    let is_embedded = anythingllm
        .update_embeddings(&workspace.slug, docs, UpdateParameter::Adds)
        .await;

    match is_embedded {
        Ok(_) => {
            sp.finish_ok("Embedding OK".to_string());
        }
        Err(_) => {
            sp.finish_error("Error embedding".to_string());
            return Ok(());
        }
    }

    // [6/6] Report failed uploads

    let failed_docs_guard = failed_docs.lock().await;
    let failed_docs = &*failed_docs_guard;

    println!("\n");

    if failed_docs.is_empty() {
        println!("All PDFs uploaded successfully");
    } else {
        println!("The following PDFs failed to upload:");

        for doc in failed_docs.iter() {
            println!("- {}", doc.display());
        }

        // create a filename with the name 'log_YYYY-MM-DD_HH-MM-SS.txt'
        let dirs = directories_next::ProjectDirs::from("com", "richardlyon", "aza").unwrap();
        let filename = format!(
            "log_{}.txt",
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
        );
        let log_path = dirs.config_dir().join(filename);
        let mut log_file = std::fs::File::create(&log_path).unwrap();
        for doc in failed_docs.iter() {
            writeln!(log_file, "{}", doc.display()).unwrap();
        }
        println!("\nFailed uploads logged to {}", log_path.display());
    }

    return Ok(());
}

// Generate a list of PathBuf to PDFs in a directory
async fn get_pdfs_from_directory(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut pdf_files = Vec::new();

    let mut dir = fs::read_dir(path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "pdf" {
            pdf_files.push(path);
        }
    }

    Ok(pdf_files)
}

#[allow(dead_code)]
struct Spinner {
    prefix: String,
    message: String,
    spinner: ProgressBar,
}

impl Spinner {
    fn new(prefix: String, message: String) -> Spinner {
        let spinner_style = ProgressStyle::default_spinner()
            // .template("{prefix:5.dim} {spinner:.green} {msg}")
            .template("{spinner:.green} {msg}")
            .unwrap();

        let spinner = ProgressBar::new_spinner()
            // .with_prefix(prefix.clone())
            .with_message(message.clone())
            .with_style(spinner_style.clone());
        spinner.enable_steady_tick(Duration::from_millis(100));

        Spinner {
            prefix: prefix.to_string(),
            message: message.to_string(),
            spinner,
        }
    }

    fn finish_ok(&self, message: String) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                // .template("{prefix:5.dim.bold} {spinner:.green} {msg:.green}")
                .template("{spinner:.green} {msg:.green}")
                .unwrap(),
        );
        self.spinner.finish_with_message(message);
    }

    fn finish_error(&self, message: String) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                // .template("{prefix:5.dim.bold} {spinner:.green} {msg:.red}")
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
