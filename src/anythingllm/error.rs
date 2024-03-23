//! Custom error types for the `anythingllm` library

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
    #[error("Workspace '{0}' not found")]
    WorkspaceIdError(String),
    #[error("Multiple workspaces with name {0} found")]
    MultipleWorkspacesError(String),

    #[error("Document exists: {0}")]
    DocumentExistsError(String),
    #[error("Document not found on filesystem")]
    FileSystemError(#[from] std::io::Error),
    #[error("Document not found in workspace: {0}")]
    DocumentNotFoundWorkspaceError(String),
    #[error("Failed to load PDF")]
    PDFLoadError(#[from] lopdf::Error),
    #[error("Multipart form encoding error: {0}")]
    MultipartFormError(String),
    #[error("File too large")]
    FileTooLarge,

    #[error("Custom error: {0}")]
    CustomError(String),
    #[error("Unhandled error: {0}")]
    UnhandledError(String),

    #[error("Cancelled")]
    Cancelled,
}
