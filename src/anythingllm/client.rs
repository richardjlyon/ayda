/// The `AnythingLLMClient` struct represents a client for the AnythingLLM API.
/// It includes the base URL for the API and a `reqwest::Client` for making requests.
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{multipart, Client, Error, StatusCode};
use serde::de::DeserializeOwned;

pub struct AnythingLLMClient {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl AnythingLLMClient {
    pub fn new(server_ip: &str, port: &str, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        let header_value = format!("Bearer {}", api_key);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&header_value).unwrap(),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self {
            base_url: format!("http://{}:{}/api/v1", server_ip, port).to_string(),
            client,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let res = self.client.get(&url).send().await?;
        let data = res.json::<T>().await?;
        Ok(data)
    }

    pub async fn post_multipart(
        &self,
        endpoint: &str,
        form: multipart::Form,
    ) -> Result<(), (StatusCode, &'static str)> {
        let url = format!("{}/{}", self.base_url, endpoint);
        println!("URL: {}", url);

        let res = self.client.post(&url).multipart(form).send().await.unwrap();

        match res.status() {
            StatusCode::OK => Ok(()),
            status => Err((status, "HTTP error")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let a = AnythingLLMClient::new("10.13.10.8", "3001", "api_key");
        assert_eq!(a.base_url, "http://10.13.10.8:3001/api/v1");
    }
}
