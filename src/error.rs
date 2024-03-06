//! Custom error types for the library.

pub type Result<T> = core::result::Result<T, LLMError>;

#[derive(thiserror::Error, Debug)]
pub enum LLMError {
    #[error("Authentication failed: {0}")]
    AuthFail(String),
    #[error("Service error: {0}")]
    ServiceError(String),
}

impl From<reqwest::Error> for LLMError {
    fn from(err: reqwest::Error) -> LLMError {
        LLMError::ServiceError(err.to_string())
    }
}
