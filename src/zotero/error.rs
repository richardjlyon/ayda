//! Custom error types for the Zotero library

use std::env::VarError;

use crate::anythingllm::error::LLMError;

#[derive(thiserror::Error, Debug)]
pub enum ZoteroError {
    #[error("Unhandled error")]
    UnhandledError(String),
    #[error("Invalid collection id")]
    InvalidCollectionId(u8),
    #[error("Invalid workspace id")]
    InvalidWorkspaceId(u8),
    #[error("Environment variable error")]
    EnvVarError(#[from] VarError),
    #[error("Invalid input: {0}")]
    InvalidInput(u8),
    #[error("PDF path error error: {0}")]
    PDFPathError(String),
    #[error("Deserialisation error")]
    DeserializationError,
}

impl From<reqwest::Error> for ZoteroError {
    fn from(error: reqwest::Error) -> ZoteroError {
        ZoteroError::UnhandledError(error.to_string())
    }
}

impl From<LLMError> for ZoteroError {
    fn from(error: LLMError) -> Self {
        ZoteroError::UnhandledError(error.to_string())
    }
}

// impl From<VarError> for ZoteroError {
//     fn from(error: VarError) -> ZoteroError {
//         ZoteroError::UnhandledError(error.to_string())
//     }
// }
