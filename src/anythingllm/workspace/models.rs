use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::anythingllm::workspace::ChatMode;

/// Represents a workspace.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub slug: String,
    #[serde(rename = "createdAt", deserialize_with = "deserialize_utc_date")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "lastUpdatedAt", deserialize_with = "deserialize_utc_date")]
    pub last_updated_at: DateTime<Utc>,
    #[serde(rename = "similarityThreshold")]
    pub similarity_threshold: f64,
    #[serde(rename = "chatMode")]
    pub chat_mode: ChatMode,
    pub documents: Option<Vec<WorkspaceDocument>>,
}

/// Represents a workspace document.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct WorkspaceDocument {
    pub id: u32,
    #[serde(rename = "docId")]
    pub doc_id: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: u32,
    #[serde(rename = "createdAt", deserialize_with = "deserialize_utc_date")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "lastUpdatedAt", deserialize_with = "deserialize_utc_date")]
    pub last_updated_at: DateTime<Utc>,
    pub docpath: String,
    pub filename: String,
    pub pinned: bool,
    #[serde(deserialize_with = "deserialize_meta_json")]
    pub metadata: WorkspaceDocumentMetadata,
}

/// Represents the metadata of a workspace document.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct WorkspaceDocumentMetadata {
    pub id: String,
    pub url: String,
    pub title: String,
    #[serde(rename = "docAuthor")]
    pub doc_author: String,
    #[serde(rename = "docSource")]
    pub doc_source: String,
    #[serde(rename = "chunkSource")]
    pub chunk_source: String,
    pub description: String,
    // #[serde(deserialize_with = "deserialize_utc_date")]
    // pub published: DateTime<Utc>,
    #[serde(rename = "wordCount")]
    pub word_count: u32,
}

/// Structure to deserialize the response from the API.
#[derive(Debug, serde::Deserialize)]
pub struct GetWorkspacesResponse {
    pub workspaces: Vec<Workspace>,
}

/// Structure to deserialize the response from the API.
#[derive(Debug, serde::Deserialize)]
pub struct GetWorkspaceNewResponse {
    pub workspace: Workspace,
    pub message: Option<String>,
}

/// Structure to deserialize the response from the API.
#[derive(Debug, serde::Deserialize)]
pub struct GetWorkspaceSlugResponse {
    pub workspace: Option<Workspace>,
}

/// Structure to deserialize the response from the API.
#[derive(Debug, serde::Deserialize, Serialize)]
pub struct PostWorkspaceSlugChatResponse {
    pub id: Option<String>,
    pub close: bool,
    pub error: Option<bool>,
    pub sources: Vec<Source>,
    #[serde(rename = "textResponse")]
    pub text_response: String,
    #[serde(rename = "type")]
    pub response_type: String,
    #[serde(rename = "chatId")]
    pub chat_id: Option<u32>,
}

/// Represents a document source.
#[derive(Debug, serde::Deserialize, Serialize)]
pub struct Source {
    #[serde(rename = "chunkSource")]
    pub chunk_source: String,
    pub description: String,
    #[serde(rename = "docAuthor")]
    pub doc_author: String,
    #[serde(rename = "docSource")]
    pub doc_source: String,
    pub id: String,
    pub published: String,
    pub score: f64,
    pub text: String,
    pub title: String,
    pub token_count_estimate: u32,
    pub url: String,
    #[serde(rename = "wordCount")]
    pub word_count: u32,
}

fn deserialize_utc_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    match DateTime::parse_from_rfc3339(&date_str) {
        Ok(datetime) => Ok(datetime.into()),
        Err(_) => Err(serde::de::Error::custom("Invalid date format")),
    }
}

fn deserialize_meta_json<'de, D>(deserializer: D) -> Result<WorkspaceDocumentMetadata, D::Error>
where
    D: Deserializer<'de>,
{
    let meta_str = String::deserialize(deserializer)?;
    match serde_json::from_str(&meta_str) {
        Ok(meta) => Ok(meta),
        Err(_) => Err(serde::de::Error::custom("Invalid metadata format")),
    }
}
