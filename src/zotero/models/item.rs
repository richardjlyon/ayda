use crate::zotero::client::ZoteroClient;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemData {
    pub key: String,
    pub filename: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub data: ItemData,
}

impl ZoteroClient {
    pub async fn fetch_items_with_pdfs(
        &self,
        collection_key: &str,
    ) -> Result<Vec<Item>, Error> {
        let url = format!("{}/collections/{}/items", self.base_url, collection_key);

        let params = [
            ("itemType", "attachment"),
            ("format", "json"),
            ("linkMode", "imported_file"),
            ("limit", "100"),
        ];

        let res = self
            .client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json::<Vec<Item>>()
            .await?;

        // filter to select content type "application/pdf"
        let pdfs = res
            .iter()
            .filter(|&x| x.data.content_type.as_ref() == Some(&"application/pdf".to_string()))
            .cloned()
            .collect::<Vec<_>>();

        Ok(pdfs)
    }
}
