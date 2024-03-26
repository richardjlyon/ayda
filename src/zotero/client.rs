//! A client for the Zotero API (v3)
//!
//! see: [Zotero Web API v3](https://www.zotero.org/support/dev/web_api/v3/start).
//!
//! The client is a simple wrapper around the reqwest client, with a few convenience methods for
//! making requests to the Zotero API.

use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::zotero::error::ZoteroError;

use super::item::models::ItemUpdateData;

/// A client for the Zotero API
#[derive(Debug)]
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

    pub async fn get(
        &self,
        endpoint: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let params = params.unwrap_or_default();
        self.client.get(&url).query(&params).send().await
    }

    /// Get an endpoint and deserialize
    pub async fn get_deserialized<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<T, ZoteroError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let params = params.unwrap_or_default(); // eta-reduction
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

    /// Patch an endpoint
    pub async fn patch(
        &self,
        endpoint: &str,
        version: i64,
        data: &ItemUpdateData,
    ) -> Result<(), ZoteroError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let json_data = serde_json::to_value(data).unwrap();

        let _ = self
            .client
            .patch(&url)
            .header("If-Unmodified-Since-Version", version)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&json_data)
            .send()
            .await;

        Ok(())
    }
}
