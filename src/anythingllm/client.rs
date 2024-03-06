use reqwest::header::{HeaderMap, HeaderValue};

pub struct AnythingLLMClient {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl AnythingLLMClient {
    pub fn new(server_ip: &str, port: u16, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        let header_value = format!("Bearer {}", api_key);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&header_value).unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            base_url: format!("http://{}:{}/api/v1", server_ip, port).to_string(),
            client,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let a = AnythingLLMClient::new("10.13.10.8", 3001, "api_key");
        assert_eq!(a.base_url, "http://10.13.10.8:3001/api/v1");
    }
}
