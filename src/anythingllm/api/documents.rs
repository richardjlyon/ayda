//! AnythingLLM API 'Documents' endpoints

use std::fs;

use regex::Regex;
use reqwest::multipart;

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::error::LLMError;
use crate::anythingllm::error::LLMError::{DocumentExistsError, DocumentNotFoundError};
use crate::anythingllm::models::document::{Document, DocumentsResponse};

impl AnythingLLMClient {
    /// List all documents
    ///
    pub async fn document_list(&self) -> Result<Vec<Document>, LLMError> {
        let response = self.get::<DocumentsResponse>("documents").await?;

        let mut documents = response.local_files.items[0]
            .items
            .as_ref()
            .unwrap()
            .iter()
            .map(|item| Document {
                id: item.id.clone().unwrap(),
                name: item.name.clone(),
                title: item.title.clone().unwrap_or_else(|| item.name.clone()),
            })
            .collect::<Vec<Document>>();

        documents.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(documents)
    }

    /// Add a new document
    ///
    pub async fn document_add(&self, file_path: &str) -> Result<Document, LLMError> {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return Err(DocumentNotFoundError(file_path.to_string()));
        }

        let file_name = filename_from_path(file_path);
        if self.document_exists(&file_name).await? {
            return Err(DocumentExistsError(file_name));
        }

        let pdf_bytes = fs::read(file_path).expect("Failed to read file");
        let pdf_part = multipart::Part::bytes(pdf_bytes)
            .file_name(file_name.clone())
            .mime_str("application/pdf")
            .expect("Invalid MIME type");

        let form = multipart::Form::new().part("file", pdf_part);
        let response = self.post_multipart("document/upload", form).await?;

        if !response.success || response.documents.is_empty() {
            return Err(LLMError::DocumentAddError(file_path.to_string()));
        }

        let document = self.find_document(&file_name).await?;
        Ok(document)
    }

    // Helper functions /////////////////////////////////////////////////////////////////////////////

    async fn document_exists(&self, file_name: &str) -> Result<bool, LLMError> {
        let documents = self.document_list().await?;
        Ok(documents
            .iter()
            .any(|doc| remove_uuid(&doc.name) == *file_name))
    }

    async fn find_document(&self, file_name: &String) -> Result<Document, LLMError> {
        let documents = self.get::<DocumentsResponse>("documents").await?;
        let matched_document = documents.local_files.items[0]
            .items
            .as_ref()
            .unwrap()
            .iter()
            .find(|item| item.title.as_ref() == Some(file_name));

        match matched_document {
            Some(document) => Ok(Document {
                id: document.id.clone().unwrap(),
                name: document.name.clone(),
                title: document
                    .title
                    .clone()
                    .unwrap_or_else(|| document.name.clone()),
            }),
            None => Err(LLMError::DocumentAddError(file_name.to_string())),
        }
    }
}

// FIXME Check whether I need this, and remove it if not
// FIXME OR move it into DocumentUploadResponseDocuments
// Wrangle the file name into the internal format used by AnythingLLM
// e.g. "Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf"
//   -> "Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf"
pub fn filename_from_path(name: &str) -> String {
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

// FIXME Check whether I need this, and remove it if not
pub fn remove_uuid(s: &str) -> String {
    let re = Regex::new(r"-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\.json$")
        .unwrap();
    re.replace(s, "").to_string()
}

// Tests /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;

    use super::*;

    struct TestFixture {
        client: AnythingLLMClient,
    }

    impl TestFixture {
        fn new() -> Self {
            dotenv().ok();
            // Setup code here. For example, initialize the AnythingLLMClient.
            let client = AnythingLLMClient::new(
                &env::var("ANYTHINGLLM_IP").expect("IP not found"),
                &env::var("ANYTHINGLLM_PORT").expect("port not found"),
                &env::var("ANYTHINGLLM_API_KEY").expect("API key not found"),
            );
            Self { client }
        }
    }

    #[tokio::test]
    async fn test_upload_document() {
        let fixture = TestFixture::new();
        let doc_path = "/Users/richardlyon/Desktop/climate pdfs/Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf";
        let result = fixture.client.document_add(doc_path).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_documents() {
        let fixture = TestFixture::new();
        let result = fixture.client.document_list().await;
        assert!(result.is_ok());

        println!("{:?}", result.unwrap());
    }

    #[test]
    fn test_name_from_path() {
        let result = filename_from_path("/Users/richardlyon/Desktop/climate pdfs/Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf");
        assert_eq!(
            result,
            "Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf"
        );
    }

    #[test]
    fn test_remove_uuid() {
        let result = remove_uuid("Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf-8f196830-2b84-41ce-b074-38c5989eb347.json");
        assert_eq!(
            result,
            "Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf"
        );
    }
}
