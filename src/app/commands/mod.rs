use crate::anythingllm::client::AnythingLLMClient;
use crate::zotero::client::ZoteroClient;
use crate::Config;

pub mod admin;
pub mod workspace;
pub mod zotero;

pub fn anythingllm_client() -> AnythingLLMClient {
    let config = Config::from_file().unwrap();
    AnythingLLMClient::new(
        &config.anythingllm_ip,
        &config.anythingllm_port,
        &config.anythingllm_api_key,
    )
}

pub fn zotero_client() -> ZoteroClient {
    let config = Config::from_file().unwrap();
    ZoteroClient::new(&config.zotero_api_key, &config.zotero_user_id)
}
