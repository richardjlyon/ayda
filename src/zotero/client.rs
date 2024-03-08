use std::fs;

use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde_json::Value;

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

    // Utility function to get the ras data for deserialisation modelling
    pub async fn get_raw(&self, endpoint: &str) -> Result<String, ZoteroError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;

        let data = response.text().await?;
        let json: Value = serde_json::from_str(&data).unwrap();
        let pretty_json = serde_json::to_string_pretty(&json).unwrap();
        fs::write("collections.json", &pretty_json).expect("Unable to write file");
        println!("DEBUG json: {}", pretty_json);

        Ok(pretty_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_client_has_correct_base_url() {
        let c = ZoteroClient::new("key", "user");
        assert_eq!(c.base_url, "https://api.zotero.org/users/user");
    }
}
