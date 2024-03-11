//! Custom error types for the AnythingLLM library.

#[derive(thiserror::Error, Debug)]
pub enum LLMError {
    #[error("Authentication failed: {0}")]
    AuthFail(String),
    #[error("Service error: {0}")]
    ServiceError(String),
    #[error("Document exists: {0}")]
    DocumentNotFoundError(String),
    #[error("Document not found: {0}")]
    DocumentExistsError(String),
    #[error("Couldn't add document: {0}")]
    DocumentAddError(String),
    #[error("No Workspace with id {0}")]
    WorkspaceIdError(u8),
    #[error("Server response fail: {0}")]
    ServerResponseFail(String),
    #[error("No documents: {0}")]
    NoDocumentsError(String),
}

impl From<reqwest::Error> for LLMError {
    fn from(err: reqwest::Error) -> LLMError {
        LLMError::ServiceError(err.to_string())
    }
}
