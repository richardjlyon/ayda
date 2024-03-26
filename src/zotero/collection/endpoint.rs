use crate::zotero::client::ZoteroClient;
use crate::zotero::collection::models::{Collection, CollectionsResponse};
use crate::zotero::error::ZoteroError;

impl ZoteroClient {
    /// GET /collections
    /// All collections in the library
    pub async fn get_collections(
        &self,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Vec<Collection>, ZoteroError> {
        let response = self
            .get_deserialized::<Vec<CollectionsResponse>>("collections", params)
            .await?;

        let collections: Vec<Collection> = response.iter().map(|c| c.data.clone()).collect();

        Ok(collections)
    }

    /// GET /collections/{collectionKey}
    /// A specific collection in the library
    pub async fn get_collections_collection_key(
        &self,
        collection_key: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Collection, ZoteroError> {
        let response = self
            .get_deserialized::<CollectionsResponse>(
                &format!("collections/{}", collection_key),
                params,
            )
            .await?;
        Ok(response.data.clone())
    }
}
