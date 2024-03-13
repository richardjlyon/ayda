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

    /// Get an endpoint and deserialize
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<T, ZoteroError> {
        let url = format!("{}/{}", self.base_url, endpoint);

        // let response = self.client.get(&url).send().await?.error_for_status()?;
        // let body = response.text().await?;
        // let json: Value = serde_json::from_str(&body).unwrap();
        // let json_pretty = serde_json::to_string_pretty(&json).unwrap();
        // std::fs::write("../../tests/responses/Zotero/response.json", json_pretty).unwrap();

        // create an empty vector of <&str, &str>
        let params = match params {
            Some(p) => p,
            None => Vec::new(),
        };

        // let mut params: Vec<(&str, &str)> = Vec::new();

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .error_for_status()?;
        let data = response.json::<T>().await?;

        Ok::<T, ZoteroError>(data)
    }
}
