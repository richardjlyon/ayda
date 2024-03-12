//! Examine the JSON returned by endpoints
#![allow(dead_code)]

use std::env;

use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{multipart, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Structs for deserializing JSON //////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub title: String,
    pub cached: bool,
}

impl From<&Item> for Document {
    fn from(item: &Item) -> Self {
        Document {
            id: item.id.clone().unwrap(),
            name: item.name.clone(),
            title: item.title.clone().unwrap(),
            cached: item.cached.unwrap(),
        }
    }
}

// derive Document from Item

#[derive(Debug, serde::Deserialize)]
struct GetWorkspaceNewResponse {
    workspace: Workspace,
    message: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct GetWorkspacesResponse {
    workspaces: Vec<Workspace>,
}

#[derive(Debug, serde::Deserialize)]
struct GetWorkspaceSlugResponse {
    workspace: Workspace,
}

#[derive(Debug, serde::Deserialize)]
struct DocumentsResponse {
    #[serde(rename = "localFiles")]
    local_files: Item,
}

#[derive(Debug, serde::Deserialize)]
struct Item {
    items: Option<Vec<Item>>,
    #[serde(rename = "type")]
    doc_type: String,
    name: String,
    title: Option<String>,
    id: Option<String>,
    description: Option<String>,
    #[serde(rename = "docAuthor")]
    doc_author: Option<String>,
    cached: Option<bool>,
}

// #[derive(Debug, Deserialize)]
// struct DocumentUploadResponse {
//     pub success: bool,
//     pub error: Option<String>,
//     pub documents: Vec<Document>,
// }
//
// #[derive(Debug, serde::Deserialize)]
// pub struct Document {
//     pub id: Option<String>,
//     pub name: Option<String>,
//     pub title: Option<String>,
//     pub cached: Option<bool>,
// }

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    authenticated: bool,
}

// Client for the AnythingLLM API //////////////////////////////////////////////////////////////////

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(ip: &str, port: &str, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
        let base_url = format!("http://{}:{}/api/v1", ip, port);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { base_url, client }
    }

    pub async fn get_auth(&self) -> std::result::Result<bool, LLMError> {
        let response = match self.get("auth").await {
            Ok(response) => response,
            Err(_) => return Err(LLMError::AuthError),
        };

        let result = response
            .json::<AuthResponse>()
            .await
            .expect("FIXME failed to parse json");

        match result.authenticated {
            true => Ok(true),
            false => Err(LLMError::AuthError),
        }
    }

    // Workspace endpoints ////////////////////////////////////////////////////////////////////////

    pub async fn post_workspace_new(&self, name: &str) -> Result<Workspace, LLMError> {
        let response = self
            .client
            .post(&format!("{}/{}", self.base_url, "workspace/new"))
            .header("Content-Type", "application/json")
            .body(json!({ "name": name }).to_string())
            .send()
            .await?
            .error_for_status()?;

        let workspace_new_response = response.json::<GetWorkspaceNewResponse>().await?;

        Ok(workspace_new_response.workspace)
    }

    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, LLMError> {
        let response = self.get("workspaces").await?.error_for_status()?;
        let workspaces_response = response.json::<GetWorkspacesResponse>().await?;

        Ok(workspaces_response.workspaces)
    }

    pub async fn get_workspace_slug(&self, slug: &str) -> Result<Workspace, LLMError> {
        let url = format!("{}/{}", "workspace", slug);
        let response = match self.get(&url).await {
            Ok(response) => response,
            Err(e) => return Err(LLMError::ServiceError(e.to_string())),
        };

        match response.json::<GetWorkspaceSlugResponse>().await {
            Ok(workspace_slug_response) => Ok(workspace_slug_response.workspace),
            Err(err) => {
                if err.is_decode() {
                    Err(LLMError::CustomError(
                        "Invalid response from server: expected struct Workspace, got null"
                            .to_string(),
                    ))
                } else {
                    Err(LLMError::ServiceError(err.to_string()))
                }
            }
        }
    }

    pub async fn delete_workspace_slug(&self, slug: &str) -> Result<(), LLMError> {
        let url = format!("{}/{}/{}", self.base_url, "workspace", slug);
        let response = match self.client.delete(&url).send().await {
            Ok(response) => response,
            Err(e) => return Err(LLMError::ServiceError(e.to_string())),
        };

        // NOTE: For a bad request, the API returns a "200 OK" status with the text "Bad Request"
        // not a "400 Bad Request"
        let response_text = response.text().await?;

        if response_text == "Bad Request" {
            return Err(LLMError::BadRequest(url));
        }

        Ok(())
    }

    // Document endpoints /////////////////////////////////////////////////////////////////////////

    pub async fn get_documents(&self) -> Result<Vec<Document>, LLMError> {
        let response = self.get("documents").await?.error_for_status()?;
        let documents_response = response.json::<DocumentsResponse>().await?;

        // documents_response is a nested list of folder and document types:
        // extract the documents from the response
        let mut documents: Vec<Document> = Vec::new();
        if let Some(items) = &documents_response.local_files.items {
            for item in items {
                process_item(item, &mut documents);
            }
        }

        Ok(documents)
    }

    // Upload a new file to AnythingLLM to be parsed and prepared for embedding
    // async fn post_document_upload(&self, file_path: &str) -> Result<Document, LLMError> {
    //     let path = std::path::Path::new(file_path);
    //
    //     if !path.exists() {
    //         return Err(LLMError::DocumentNotFoundFileSystemError(
    //             file_path.to_string(),
    //         ));
    //     }
    //
    //     let file_name = filename_from_path(file_path);
    //     if self.document_exists(&file_name).await? {
    //         return Err(LLMError::DocumentExistsError(file_name));
    //     }
    //
    //     let pdf_bytes = fs::read(file_path).expect("Failed to read file");
    //     let pdf_part = multipart::Part::bytes(pdf_bytes)
    //         .file_name(file_name.clone())
    //         .mime_str("application/pdf")
    //         .expect("Invalid MIME type");
    //
    //     let form = multipart::Form::new().part("file", pdf_part);
    //
    //     let response = self.post_multipart("document/upload", form).await?;
    //
    //     if !response.success {
    //         return Err(LLMError::ServiceError(file_path.to_string()));
    //     }
    //
    //     if response.documents.is_empty() {
    //         return Err(LLMError::DocumentNotFoundWorkspaceError(
    //             file_path.to_string(),
    //         ));
    //     }
    //
    //     let document = self.find_document(&file_name).await?;
    //     Ok(document)
    // }

    // Private methods ////////////////////////////////////////////////////////////////////////////

    async fn get(&self, endpoint: &str) -> Result<Response, LLMError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .get(url.clone())
            .send()
            .await?
            .error_for_status()?;

        Ok(response)
    }

    // async fn post_multipart(
    //     &self,
    //     endpoint: &str,
    //     form: multipart::Form,
    // ) -> Result<DocumentUploadResponse, LLMError> {
    //     let response = self
    //         .client
    //         .post(&format!("{}/{}", self.base_url, endpoint))
    //         .multipart(form)
    //         .send()
    //         .await
    //         .unwrap();
    //
    //     if response.status() != StatusCode::OK {
    //         return Err(LLMError::ServiceError(response.status().to_string()));
    //     }
    //
    //     let document_response: DocumentUploadResponse = response.json().await?;
    //     Ok(document_response)
    // }

    // Return true if the document with the given file name exists in AnythingLLM
    // async fn document_exists(&self, file_name: &str) -> Result<bool, LLMError> {
    //     let documents = self.document_list().await?;
    //     Ok(documents
    //         .iter()
    //         .any(|doc| remove_uuid(&doc.name) == *file_name))
    // }

    //
    // async fn find_document(&self, file_name: &String) -> Result<Document, LLMError> {
    //
    //     let documents = self.get::<DocumentsResponse>("documents").await?;
    //
    //
    //     let matched_document = documents.local_files.items[0]
    //         .items
    //         .as_ref()
    //         .unwrap()
    //         .iter()
    //         .find(|item| item.title.as_ref() == Some(file_name));
    //
    //     match matched_document {
    //         Some(document) => Ok(Document {
    //             id: document.id.clone().unwrap(),
    //             name: document.name.clone(),
    //             title: document
    //                 .title
    //                 .clone()
    //                 .unwrap_or_else(|| document.name.clone()),
    //             cached: None,
    //         }),
    //         None => Err(LLMError::UnhandledError(file_name.to_string())),
    //     }
    // }
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

fn remove_uuid(s: &str) -> String {
    let re = Regex::new(r"-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\.json$")
        .unwrap();
    re.replace(s, "").to_string()
}

fn process_item(item: &Item, documents: &mut Vec<Document>) {
    if item.doc_type == "file" {
        documents.push(item.into());
    }

    if let Some(nested_items) = &item.items {
        for nested_item in nested_items {
            process_item(nested_item, documents);
        }
    }
}

// Errors //////////////////////////////////////////////////////////////////////////////////////////

#[derive(thiserror::Error, Debug)]
pub enum LLMError {
    #[error("Authentication error")]
    AuthError,
    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("Document exists: {0}")]
    DocumentExistsError(String),
    #[error("Document not found on filesystem: {0}")]
    DocumentNotFoundFileSystemError(String),
    #[error("Document not found in workspace: {0}")]
    DocumentNotFoundWorkspaceError(String),

    #[error("Custom error: {0}")]
    CustomError(String),
    #[error("Unhandled error: {0}")]
    UnhandledError(String),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fixtures ////////////////////////////////////////////////////////////////////////////////////

    struct Fixture {
        client: Client,
        workspace: Workspace,
    }

    impl Fixture {
        async fn new() -> Self {
            dotenv::dotenv().ok();
            let api_key = &env::var("ANYTHINGLLM_API_KEY").expect("API key not found");
            let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
            let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
            let client = Client::new(ip, port, api_key);

            // Create a test workspace
            let uuid = uuid::Uuid::new_v4();
            let workspace_name = format!("DELETE ME {}", uuid);
            let workspace = client.post_workspace_new(&workspace_name).await.unwrap();

            Self { client, workspace }
        }

        async fn remove(self) {
            let _ = &self
                .client
                .delete_workspace_slug(&self.workspace.slug)
                .await;
        }
    }

    // Client tests ///////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_client_new() {
        dotenv::dotenv().ok();
        let api_key = "api_key";
        let ip = "10.13.10.8";
        let port = "3001";
        let client = Client::new(ip, port, api_key);

        assert_eq!(client.base_url, "http://10.13.10.8:3001/api/v1");
    }

    #[tokio::test]
    async fn test_get_auth_ok() {
        dotenv::dotenv().ok();
        let api_key = &env::var("ANYTHINGLLM_API_KEY").expect("API key not found");
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = Client::new(ip, port, api_key);

        assert!(client.get_auth().await.is_ok());
    }

    #[tokio::test]
    async fn test_get_auth_err() {
        dotenv::dotenv().ok();
        let api_key = "INVALID_API_KEY";
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = Client::new(ip, port, api_key);

        assert!(client.get_auth().await.is_err());
    }

    // Workspace tests ////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_post_workspace_new() {
        let fixture = Fixture::new().await;
        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_workspaces() {
        let fixture = Fixture::new().await;
        let workspaces = fixture.client.get_workspaces().await.unwrap();
        let workspace_slug = &fixture.workspace.slug;

        assert!(workspaces.len() > 0);
        assert!(workspaces
            .iter()
            .any(|w| w.slug == workspace_slug.to_string()));

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_workspace_slug() {
        let fixture = Fixture::new().await;
        let test_workspace_slug = &fixture.workspace.slug;
        let workspace = fixture
            .client
            .get_workspace_slug(&test_workspace_slug)
            .await
            .unwrap();

        assert_eq!(workspace.slug, test_workspace_slug.to_string());

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_workspace_invalid_slug() {
        let fixture = Fixture::new().await;
        let workspace = fixture
            .client
            .get_workspace_slug("invalid-workspace-slug")
            .await;

        println!("{:?}", workspace);

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_delete_workspace_slug() {
        let fixture = Fixture::new().await;
        let test_workspace_slug = &fixture.workspace.slug;
        let _ = fixture
            .client
            .delete_workspace_slug(&test_workspace_slug)
            .await
            .unwrap();
        let workspaces = fixture.client.get_workspaces().await.unwrap();

        assert!(!workspaces
            .iter()
            .any(|w| w.slug == test_workspace_slug.to_string()));
    }

    #[tokio::test]
    async fn test_delete_workspace_invalid_slug() {
        let fixture = Fixture::new().await;
        let response = fixture
            .client
            .delete_workspace_slug("invalid-workspace-slug")
            .await;

        match response {
            Ok(_) => panic!("Expected an error, but got Ok(_)"),
            Err(err) => match err {
                LLMError::BadRequest(_) => (), // Test passes if we get here
                _ => panic!("Expected LLMError::BadRequest, but got a different error"),
            },
        }

        fixture.client.get_workspaces().await.unwrap();
    }

    #[tokio::test]
    async fn test_workspace_slug_update_embeddings() {}

    // Document tests /////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_documents() {
        let fixture = Fixture::new().await;

        let docs = fixture.client.get_documents().await;
        println!("docs: {:#?}", docs);

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_post_document_upload() {
        // let fixture = Fixture::new().await;
        // let file_path =
        //     "/Users/richardlyon/Zotero/storage/2CPPXSFP/Vogler (2001) Future Directions.pdf";
        // let doc = fixture
        //     .client
        //     .post_document_upload(&file_path)
        //     .await
        //     .unwrap();
        //
        // // confirm the document is in the workspace
        //
        // fixture.remove().await;
    }

    #[tokio::test]
    async fn test_post_document_upload_invalid() {
        // let fixture = Fixture::new().await;
        //
        // // upload a test document
        // let doc = fixture.client.post_document_upload().await.unwrap();
        // // confirm the document is in the workspace
        // // delete the workspace
        //
        // fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_document_docname() {}

    #[tokio::test]
    async fn test_get_document_invalid_docname() {}
}
