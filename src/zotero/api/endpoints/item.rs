use reqwest::Error;

use crate::zotero::api::models::item::Item;
use crate::zotero::client::ZoteroClient;

// pub use api::*;
// use crate::zotero::api::models::item::Item;
// use crate::zotero::client::ZoteroClient;

impl ZoteroClient {
    pub async fn pdf_items(&self, collection_key: &str) -> Result<Vec<Item>, Error> {
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
