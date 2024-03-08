pub mod document;
pub mod workspace;
pub mod zotero;

use crate::anythingllm::client::AnythingLLMClient;
use crate::zotero::client::ZoteroClient;
use std::env;
pub use workspace::workspace_list;

pub fn anythingllm_client() -> AnythingLLMClient {
    dotenv::dotenv().ok();
    AnythingLLMClient::new(
        &env::var("ANYTHINGLLM_IP").expect("IP not found"),
        &env::var("ANYTHINGLLM_PORT").expect("port not found"),
        &env::var("ANYTHINGLLM_API_KEY").expect("API key not found"),
    )
}

pub fn zotero_client() -> ZoteroClient {
    dotenv::dotenv().ok();
    ZoteroClient::new(
        &env::var("ZOTERO_API_KEY").expect("Zotero API key not found"),
        &env::var("ZOTERO_USER_ID").expect("Zotero User ID not found"),
    )
}
