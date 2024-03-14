//! Zotero API '/items' endpoints

use crate::zotero::api::models::item::{Item, ItemsResponse};
use crate::zotero::client::ZoteroClient;
use crate::zotero::error::ZoteroError;

impl ZoteroClient {
    /// Get /items
    /// All items in the library excluding trashed items
    pub async fn get_items(
        &self,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Vec<Item>, ZoteroError> {
        let response = self.get::<Vec<ItemsResponse>>("items", params).await?;
        let items = response.iter().map(|c| c.data.clone()).collect();

        Ok(items)
    }

    /// GET /items/{itemKey}
    /// A specific item in the library
    pub async fn get_items_item_key(
        &self,
        item_key: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Item, ZoteroError> {
        let response = self
            .get::<ItemsResponse>(&format!("items/{}", item_key), params)
            .await?;
        Ok(response.data.clone())
    }

    /// GET /collections/{collectionKey}/items
    /// All items in a specific collection, with optional search parameters
    pub async fn get_collections_collection_key_items(
        &self,
        collection_key: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Vec<Item>, ZoteroError> {
        let response = self
            .get::<Vec<ItemsResponse>>(&format!("collections/{}/items", collection_key), params)
            .await?;

        let items = response.iter().map(|c| c.data.clone()).collect();

        Ok(items)

        // Ok(res.into_iter().filter(|x| x.is_pdf()).collect())
    }
}
