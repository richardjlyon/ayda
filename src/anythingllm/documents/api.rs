use std::fs;
use std::path::Path;

use regex::Regex;
use reqwest::multipart;
use reqwest::multipart::Form;
use tempfile::NamedTempFile;

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::documents::{Document, DocumentMultipartResponse, DocumentsResponse, Item};
use crate::anythingllm::error::LLMError;

impl AnythingLLMClient {
    /// GET /documents
    pub async fn get_documents(&self) -> Result<Vec<Document>, LLMError> {
        let response = self.get("documents").await?.error_for_status()?;

        let documents_response = response.json::<DocumentsResponse>().await?;

        // documents_response is a nested list of folder and document types:
        // extract the documents from the response
        let mut documents: Vec<Document> = Vec::new();
        if let Some(items) = &documents_response.local_files.items {
            for item in items {
                Self::process_item(item, &mut documents);
            }
        }

        Ok(documents)
    }

    /// POST /document/upload
    pub async fn post_document_upload(&self, file_path: &str) -> Result<Document, LLMError> {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return Err(LLMError::FileSystemError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File not found",
            )));
        }

        let new_title = Self::make_pdf_meta_title(path);
        let mut doc = Self::set_pdf_meta_title(file_path, new_title)?;
        let temp_file_path = NamedTempFile::new()?;
        doc.save(&temp_file_path).unwrap();

        let form = Self::create_multipart_form(&temp_file_path, file_path)?;
        let response = self.post_multipart("document/upload", form).await?;
        if !response.status().is_success() {
            return Err(LLMError::ServiceError(file_path.to_string()));
        }
        let document = (&response
            .json::<DocumentMultipartResponse>()
            .await?
            .documents[0]
            .clone())
            .into();

        Ok(document)
    }

    // helper functions ///////////////////////////////////////////////////////////////////////

    fn make_pdf_meta_title(path: &Path) -> String {
        let new_title = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map(|s| s.to_string())
            .unwrap();
        new_title
    }

    // Set the title of a PDF file at 'file_path' to 'new_title'
    fn set_pdf_meta_title(file_path: &str, new_title: String) -> Result<lopdf::Document, LLMError> {
        let mut doc = lopdf::Document::load(file_path)?;

        for _ in doc.traverse_objects(|x| {
            let Some(Ok(title)) = x
                .as_dict_mut()
                .ok()
                .and_then(|d| d.get_mut(b"Title").ok())
                .map(|o| o.as_str_mut())
            else {
                return;
            };
            // let new_title = "This is a bollocks title";
            title.clear();
            title.extend_from_slice(new_title.as_bytes());
        }) {}
        Ok(doc)
    }

    // Create a multipart form with a PDF file
    fn create_multipart_form(
        temp_file_path: &NamedTempFile,
        file_path: &str,
    ) -> Result<Form, LLMError> {
        let file_name = Self::filename_from_path(file_path);
        let pdf_bytes = fs::read(temp_file_path)?;
        let pdf_part = multipart::Part::bytes(pdf_bytes)
            .file_name(file_name.clone())
            .mime_str("application/pdf")?;
        let form = multipart::Form::new().part("file", pdf_part);

        Ok(form)
    }

    // Recursively process the items in the response to extract the documents
    fn process_item(item: &Item, documents: &mut Vec<Document>) {
        if item.doc_type.as_ref() == Some(&"file".to_string()) {
            documents.push(item.into());
        }

        if let Some(nested_items) = &item.items {
            for nested_item in nested_items {
                Self::process_item(nested_item, documents);
            }
        }
    }

    // Wrangle the file name into the internal format used by AnythingLLM
    // e.g. "Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf"
    //   -> "Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf"
    fn filename_from_path(name: &str) -> String {
        let path = std::path::Path::new(name);
        let file_name = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map(|s| s.to_string())
            .unwrap();

        let multi_space = Regex::new(r" +").unwrap();
        let file_name = multi_space.replace_all(&file_name, " ");

        file_name
            .replace(" - ", "-")
            .replace(',', "")
            .replace(' ', "-")
    }
}

mod tests {
    // use super::*;

    use crate::anythingllm::client::AnythingLLMClient;

    #[test]
    fn test_filename_from_path() {
        let filename =
            "Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf";
        let expected = "Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf";

        assert_eq!(AnythingLLMClient::filename_from_path(filename), expected);
    }
}
