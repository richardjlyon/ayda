use serde::{Deserialize, Serialize};


// v1/document/upload
#[derive(Debug, Deserialize)]
pub struct DocumentUploadResponse {
    pub success: bool,
    pub error: Option<String>,
    pub documents: Vec<DocumentUploadResponseDocuments>,
}

#[derive(Debug, Deserialize)]
pub struct DocumentUploadResponseDocuments {
    pub id: String,
}

// v1/documents

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentsResponse {
    #[serde(rename = "localFiles")]
    pub local_files: DocumentsResponseLocalFiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentsResponseLocalFiles {
    pub items: Vec<DocumentsResponseItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentsResponseItem {
    pub name: String,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub items: Option<Vec<DocumentsResponseItem>>,
    pub title: Option<String>,
}
