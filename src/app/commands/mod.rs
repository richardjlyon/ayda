pub mod document;
pub mod workspace;

use crate::anythingllm::client::AnythingLLMClient;
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
