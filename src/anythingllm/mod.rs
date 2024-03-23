//! AnythingLLM client library.
//!
//! This library provides a client for the AnythingLLM server. For API documentation, see the
//! [documentation](http://localhost:3001/api/docs/) on your local server.
//!

pub use documents::*;
pub use workspace::*;

pub mod client;
pub mod documents;
pub mod error;
pub mod workspace;
