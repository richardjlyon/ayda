//! `Document` models and endpoints for the `anythingllm` client.
//!
//! ## Example usage:
//!
//! ```rust
//! use anythingllm::client::AnythingLLMClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = AnythingLLMClient::new("127.0.0.1", "8080", "api_key");
//!     let docs = client.get_documents().await.unwrap();
//!     assert!(docs.len() > 0);
//! }
//! ```
//!
#[allow(unused_imports)]
pub use endpoint::*;
pub use models::*;

mod endpoint;
mod models;
