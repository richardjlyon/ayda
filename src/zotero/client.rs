use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Error;
use serde::de::DeserializeOwned;

pub struct ZoteroClient {
    pub client: reqwest::Client,
    pub base_url: String,
}

impl ZoteroClient {
    pub fn new(api_key: &str, user_id: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Zotero-API-Key", HeaderValue::from_str(api_key).unwrap());
        headers.insert("Zotero-API-Version", HeaderValue::from_static("3"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            client,
            base_url: format!("https://api.zotero.org/users/{}", user_id).to_string(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let res = self.client.get(&url).send().await?;
        let data = res.json::<T>().await?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = ZoteroClient::new("key", "user");
        assert_eq!(c.base_url, "https://api.zotero.org/users/user");
    }
}
