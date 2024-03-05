use reqwest::header::{HeaderMap, HeaderValue};

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
