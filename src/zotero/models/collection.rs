use crate::zotero::client::ZoteroClient;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionData {
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub data: CollectionData,
}

impl ZoteroClient {
    pub async fn fetch_collections(&self) -> Result<Vec<Collection>, Error> {
        let url = format!("{}/collections", self.base_url);

        let res = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<Collection>>()
            .await?;

        Ok(res)
    }
}
