//! Zotero API 'Collections' endpoints

use crate::zotero::api::models::collection::{CollectionResponse, CollectionResponseData};
use crate::zotero::client::ZoteroClient;
use crate::zotero::error::ZoteroError;

impl ZoteroClient {
    /// Get all collections
    pub async fn zotero_list(&self) -> Result<Vec<CollectionResponseData>, ZoteroError> {
        let response = self.get::<Vec<CollectionResponse>>("collections").await?;

        Ok(response.iter().map(|c| c.data.clone()).collect())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn zotero_list_returns_collections() {
//         let c = ZoteroClient::new("key", "user");
//         let collections = c.zotero_list().await.unwrap();
//         assert!(collections.len() > 0);
//     }
// }
