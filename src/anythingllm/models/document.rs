use crate::anythingllm::client::AnythingLLMClient;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: String,
    id: Option<String>,
    #[serde(rename = "type")]
    item_type: String,
    items: Option<Vec<Item>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalFiles {
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentResponse {
    #[serde(rename = "localFiles")]
    local_files: LocalFiles,
}

impl AnythingLLMClient {
    pub async fn documents(&self) -> Result<Vec<Item>, Error> {
        let documents = self
            .client
            .get(format!("{}/documents", self.base_url))
            .send()
            .await?
            .json::<DocumentResponse>()
            .await?
            .local_files
            .items;

        Ok(documents)
    }
}
