//! AnythingLLM API 'Documents' endpoints

use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::error::LLMError::DocumentExistsError;
use crate::anythingllm::error::{LLMError, Result};
use regex::Regex;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::fs;

/// Represents a Document object in the AnythingLLM API
#[derive(Debug)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub title: String,
}

impl AnythingLLMClient {
    /// Add a new document
    pub async fn document_add(&self, file_path: &str) -> Result<()> {
        // Check the document doesn't already exist
        let documents = self.document_list().await.unwrap();
        let file_name = name_from_path(file_path);
        for doc in documents {
            let doc_name = remove_uuid(&doc.name);
            if file_name == doc_name {
                return Err(DocumentExistsError(file_name));
            }
        }

        // Read the PDF file into a Vec<u8>
        let pdf_bytes = fs::read(file_path).expect("Failed to read file");

        // Create a Part from the PDF bytes
        let pdf_part = multipart::Part::bytes(pdf_bytes)
            .file_name(name_from_path(file_path))
            .mime_str("application/pdf")
            .expect("Invalid MIME type");

        // Create a Form and add the Part to it
        let form = multipart::Form::new().part("file", pdf_part);

        let response = self.post_multipart("document/upload", form).await.unwrap();
        println!("{:#?}", response);

        // if !response.success {
        //     return Err(LLMError::DocumentAddError(response));
        // }

        Ok(())
    }
    /// Get all documents
    pub async fn document_list(&self) -> Result<Vec<Document>> {
        match self.get::<DocumentResponse>("documents").await {
            Ok(response) => Ok(response.local_files.items[0]
                .items
                .as_ref()
                .unwrap()
                .iter()
                .filter_map(|item| {
                    item.id.clone().map(|id| Document {
                        id,
                        name: item.name.clone(),
                        title: item.title.clone().unwrap_or_else(|| item.name.clone()),
                    })
                })
                .collect()),
            Err(e) => Err(LLMError::ServiceError(e.to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DocumentResponse {
    #[serde(rename = "localFiles")]
    local_files: LocalFiles,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocalFiles {
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    id: Option<String>,
    #[serde(rename = "type")]
    item_type: String,
    items: Option<Vec<Item>>,
    title: Option<String>,
}

// Utility functions /////////////////////////////////////////////////////////////////////////////

// Wrangle the file name into the internal format used by AnythingLLM
// e.g. "Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf"
//   -> "Skrable-et-al.-2022-World-Atmospheric-CO2-Its-14C-Specific-Activity-.pdf"
pub fn name_from_path(name: &str) -> String {
    let path = std::path::Path::new(name);
    let file_name = path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map(|s| s.to_string())
        .unwrap();

    let multi_space = Regex::new(r" +").unwrap();
    let file_name = multi_space.replace_all(&file_name, " ");
    let file_name = file_name
        .replace(" - ", "-")
        .replace(",", "")
        .replace(" ", "-");

    file_name
}

pub fn remove_uuid(s: &str) -> String {
    let re = Regex::new(r"-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\.json$")
        .unwrap();
    re.replace(s, "").to_string()
}

// Tests /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

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
        let result = name_from_path("/Users/richardlyon/Desktop/climate pdfs/Skrable et al. - 2022 - World Atmospheric CO2, Its 14C Specific Activity, .pdf");
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
