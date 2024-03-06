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
    pub async fn items_with_pdfs(&self, collection_key: &str) -> Result<Vec<Item>, Error> {
        let params = [
            ("itemType", "attachment"),
            ("format", "json"),
            ("linkMode", "imported_file"),
            ("limit", "100"),
        ];

        let res = self
            .client
            .get(format!(
                "{}/collections/{}/items",
                self.base_url, collection_key
            ))
            .query(&params)
            .send()
            .await?
            .json::<Vec<Item>>()
            .await?;

        Ok(res.into_iter().filter(|x| x.is_pdf()).collect())
    }
}

impl Item {
    pub fn is_pdf(&self) -> bool {
        self.data.content_type.as_deref() == Some("application/pdf")
    }

    pub fn filepath(&self, root: &str) -> Option<String> {
        self.is_pdf().then(|| {
            format!(
                "{}/{}/{}.pdf",
                root,
                self.data.key,
                self.data.filename.as_ref().unwrap()
            )
        })
    }
}
