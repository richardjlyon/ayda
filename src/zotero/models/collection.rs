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
    pub async fn collections(&self) -> Result<Vec<Collection>, Error> {
        let collections = self
            .client
            .get(format!("{}/collections", self.base_url))
            .send()
            .await?
            .json::<Vec<Collection>>()
            .await?;

        Ok(collections)
    }
}
