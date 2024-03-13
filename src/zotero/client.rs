//! A client for the Zotero API (v3)
//!
//! see: [Zotero Web API v3](https://www.zotero.org/support/dev/web_api/v3/start).
//!
//! The client is a simple wrapper around the reqwest client, with a few convenience methods for
//! making requests to the Zotero API.

use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::zotero::error::ZoteroError;

pub struct ZoteroClient {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl ZoteroClient {
    pub fn new(api_key: &str, user_id: &str) -> Self {
        let headers = [("Zotero-API-Key", api_key), ("Zotero-API-Version", "3")]
            .iter()
            .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
            .collect::<HeaderMap>();

        Self {
            base_url: format!("https://api.zotero.org/users/{}", user_id),
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, ZoteroError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;

        // TODO insert error handling here
        let data = response.json::<T>().await?;

        Ok::<T, ZoteroError>(data)
    }
}
