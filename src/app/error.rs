//! Custom errors for the CLI app

use crate::anythingllm::error::LLMError;

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("No Workspace with id: {0}")]
    WorkspaceIdError(u8),
    #[error("Document missing: {0}")]
    DocumentMissingError(String),
    #[error("Document exists: {0}")]
    DocumentExistsError(String),
    #[error("Command error: {0}")]
    CommandError(String),
}

impl From<LLMError> for AppError {
    fn from(error: LLMError) -> Self {
        match error {
            LLMError::DocumentExistsError(e) => AppError::DocumentExistsError(e.to_string()),
            _ => AppError::CommandError(error.to_string()),
        }
    }
}
