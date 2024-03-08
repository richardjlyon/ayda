//! Zotero API 'Collections' endpoints

use crate::zotero::client::ZoteroClient;
use crate::zotero::error::ZoteroError;
use crate::zotero::models::collection::{CollectionResponse, CollectionResponseData};

impl ZoteroClient {
    /// Get all collections
    pub async fn zotero_list(&self) -> Result<Vec<CollectionResponseData>, ZoteroError> {
        let response = self.get::<Vec<CollectionResponse>>("collections").await?;

        Ok(response.iter().map(|c| c.data.clone()).collect())
    }
}
