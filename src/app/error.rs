//! Custom errors for the CLI app

use crate::anythingllm::error::LLMError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("No Workspace with id {0}")]
    WorkspaceIdError(u8),
    #[error("Document not found: {0}")]
    DocumentNotFoundError(String),
    #[error("Document exists: {0}")]
    DocumentExistsError(String),
    #[error("Couldn't add document: {0}")]
    DocumentAddError(String),
    #[error("Command error: {0}")]
    CommandError(String),
}

impl From<LLMError> for AppError {
    fn from(error: LLMError) -> Self {
        match error {
            LLMError::DocumentExistsError(e) => AppError::DocumentExistsError(e.to_string()),
            LLMError::DocumentNotFoundError(e) => AppError::DocumentNotFoundError(e.to_string()),
            LLMError::WorkspaceIdError(e) => AppError::WorkspaceIdError(e),
            LLMError::DocumentAddError(e) => AppError::DocumentAddError(e),
            _ => AppError::CommandError(error.to_string()),
        }
    }
}
