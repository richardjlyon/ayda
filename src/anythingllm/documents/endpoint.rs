use std::path::Path;

use regex::Regex;
use reqwest::multipart;
use reqwest::multipart::Form;
use serde_json::{json, Value};
use tempfile::NamedTempFile;
use tracing::{event, span, Level};

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::documents::{Document, DocumentMultipartResponse, DocumentsResponse, Item};
use crate::anythingllm::error::LLMError;

// Documents API /////////////////////////////////////////////////////////////////////////////

impl AnythingLLMClient {
    /// GET /documents
    pub async fn get_documents(&self) -> Result<Vec<Document>, LLMError> {
        let documents_response = self
            .get("documents")
            .await?
            .error_for_status()?
            .json::<DocumentsResponse>()
            .await?;

        let documents: Vec<Document> = documents_response
            .local_files
            .items
            .unwrap_or_default()
            .iter()
            .flat_map(Self::extract_documents)
            .collect();

        Ok(documents)
    }

    /// POST /document/upload
    #[tracing::instrument(skip(self))]
    pub async fn post_document_upload(&self, path: &Path) -> Result<Document, LLMError> {
        if !path.exists() {
            event!(
                Level::ERROR,
                "Path does not exist: {}",
                path.to_string_lossy()
            );
            return Err(LLMError::FileSystemError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File not found",
            )));
        }

        // Fix PDF title
        let path_owned = path.to_owned();

        let parent = span::Span::current();
        let temp_file_path = tokio::task::spawn_blocking(move || {
            let inner_span = span!(parent: parent, Level::INFO, "set pdf name");
            let _inner_span_guard = inner_span.enter();

            let new_title = Self::make_pdf_meta_title(&path_owned)?;
            tracing::info!(parent: &inner_span, "setting title to {}", new_title);
            let mut doc = Self::set_pdf_meta_title(&path_owned, new_title)?;
            let temp_file_path = NamedTempFile::new()?;
            tracing::info!(parent: &inner_span, "saving file to {}", temp_file_path.path().display());
            doc.save(&temp_file_path).unwrap();
            event!(parent: &inner_span, Level::INFO, "temp file saved");

            Ok::<_, LLMError>(temp_file_path)
        })
        .await
        .unwrap()?;

        event!(Level::INFO, "creating multipart form");
        let form = Self::create_multipart_form(&temp_file_path, path).await?;

        event!(Level::INFO, "posting multipart form");
        let response = self.post_multipart("document/upload", form).await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "post error");
            return Err(LLMError::ServiceError(path.to_string_lossy().to_string()));
        }
        event!(Level::INFO, "multipart form posted");

        let document = (&response
            .json::<DocumentMultipartResponse>()
            .await?
            .documents[0]
            .clone())
            .into();

        Ok(document)
    }

    /// DELETE /api/system/remove-documents
    /// Delete documents from the repository
    /// NOTE: This is not documented in the API so possibly unstable
    pub async fn delete_api_system_remove_documents(
        &self,
        document_ids: Vec<String>,
    ) -> Result<(), LLMError> {
        // create json with key "names" and value of document_ids
        let data = json!({ "names": document_ids });

        let _ = self
            .delete("api/system/remove-documents", &data)
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// GET /api/workspace/{slug}
    /// Get the documents in the workspace with the given slug
    /// NOTE: This is not documented in the API so possibly unstable
    pub async fn get_api_workspace_slug(&self, slug: &str) -> Result<(), LLMError> {
        let url = format!("{}/api/workspace/{}", self.base_url, slug);
        dbg!(&url);
        let response = self.get(&url).await?.error_for_status()?;

        let json = response.json::<Value>().await?;

        dbg!(json);

        // let documents: Vec<Document> = response
        //     .local_files
        //     .items
        //     .unwrap_or_default()
        //     .iter()
        //     .flat_map(Self::extract_documents)
        //     .collect();

        Ok(())
    }

    // helper functions ///////////////////////////////////////////////////////////////////////

    // documents_response is a nested list of folder and document types:
    // extract the documents from the response
    fn extract_documents(item: &Item) -> Vec<Document> {
        let mut documents = Vec::new();
        if item.doc_type.as_ref() == Some(&"file".to_string()) {
            documents.push(item.into());
        }
        if let Some(nested_items) = &item.items {
            documents.extend(nested_items.iter().flat_map(Self::extract_documents));
        }
        documents
    }

    fn make_pdf_meta_title(path: &Path) -> Result<String, LLMError> {
        let new_title = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map(|s| s.to_string())
            .unwrap_or("UNKNOWN TITLE".to_string());

        Ok(new_title)
    }

    // Set the title of a PDF file at 'file_path' to 'new_title'
    #[tracing::instrument]
    fn set_pdf_meta_title(
        file_path: &Path,
        new_title: String,
    ) -> Result<lopdf::Document, LLMError> {
        const MAX_FILE_SIZE_MB: u64 = 50 * 1024 * 1024;

        let metadata = std::fs::metadata(file_path)?;
        if metadata.len() > MAX_FILE_SIZE_MB {
            tracing::error!("file too large");
            return Err(LLMError::FileTooLarge);
        }

        let mut doc = lopdf::Document::load(file_path)?;
        tracing::info!("file loaded");

        for _ in doc.traverse_objects(|x| {
            // TODO: cancellation
            // if cancel.is_cancelled() {
            //     return Err(LLMError::Cancelled);
            // }

            let Some(Ok(title)) = x
                .as_dict_mut()
                .ok()
                .and_then(|d| d.get_mut(b"Title").ok())
                .map(|o| o.as_str_mut())
            else {
                return;
            };
            title.clear();
            title.extend_from_slice(new_title.as_bytes());
        }) {}

        Ok(doc)
    }

    // Create a multipart form with a PDF file
    async fn create_multipart_form(
        temp_file_path: &NamedTempFile,
        file_path: &Path,
    ) -> Result<Form, LLMError> {
        let file_name = Self::filename_from_path(file_path);

        let pdf_file = tokio::fs::File::open(temp_file_path).await?;
        let len = pdf_file.metadata().await.unwrap().len();
        let stream = tokio_util::io::ReaderStream::new(pdf_file); // convert AsyncRead to Stream

        let pdf_part = multipart::Part::stream_with_length(reqwest::Body::wrap_stream(stream), len)
            .file_name(file_name.clone())
            .mime_str("application/pdf")?;

        let form = multipart::Form::new().part("file", pdf_part);

        Ok(form)
    }

    /// Wrangle the file name into the internal format used by AnythingLLM
    /// e.g. "Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf"
    ///   -> "Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf"
    fn filename_from_path(name: &Path) -> String {
        let file_name = name.file_name().unwrap().to_str().unwrap();

        let multi_space = Regex::new(r" +").unwrap();
        let file_name = multi_space.replace_all(file_name, " ");

        file_name
            .replace(" - ", "-")
            .replace(',', "")
            .replace(' ', "-")
    }
}

mod tests {
    #![allow(unused_imports)]

    use std::path::PathBuf;

    use crate::anythingllm::client::AnythingLLMClient;

    #[test]
    fn test_filename_from_path() {
        let filename = PathBuf::from(
            "Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf",
        );
        let expected = "Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf";

        assert_eq!(AnythingLLMClient::filename_from_path(&filename), expected);
    }

    #[test]
    fn test_set_pdf_meta_title_rejects_large_pdf() {
        let file_path = PathBuf::from("/Users/richardlyon/Zotero/storage/8EPJ2G6W/IPCC-2021-Climate Change The Physical Science Basis 2021.pdf");
        assert!(
            AnythingLLMClient::set_pdf_meta_title(&file_path, "new title".to_string()).is_err()
        );
    }
}
