use futures::StreamExt;

use crate::zotero::client::ZoteroClient;
use crate::zotero::collection::model::Collection;
use crate::zotero::error::ZoteroError;
use crate::zotero::item::model::{Item, ItemsResponse};

impl ZoteroClient {
    /// GET /items
    pub fn get_items(&self) -> impl futures::stream::Stream<Item = Item> + '_ {
        self.get_batched("items".to_string())
    }

    /// GET /collections/<collection_key>/items
    pub fn get_collections_collection_key_items_batched(
        &self,
        collection_key: String,
    ) -> impl futures::stream::Stream<Item = Item> + '_ {
        let endpoint = format!("collections/{}/items", collection_key);

        self.get_batched(endpoint)
    }

    /// Get all items in the library in batches
    fn get_batched(&self, endpoint: String) -> impl futures::stream::Stream<Item = Item> + '_ {
        const MAX_RESULTS: i32 = 2000;
        const CHUNK_SIZE: i32 = 100;
        let chunks = MAX_RESULTS / CHUNK_SIZE;

        futures::stream::iter((0..chunks).map(move |x| {
            let endpoint = endpoint.clone();
            ZoteroClient::process_batch(endpoint, x * CHUNK_SIZE, CHUNK_SIZE, self)
        }))
        .buffer_unordered(chunks as usize)
        .filter_map(|f| async { f.ok() })
        .flat_map(futures::stream::iter)
    }

    /// Return a matching collection if collection_name corresponds to exactly one workspace
    /// NOTE: Case insensitive so 'COVID' matches 'covid'
    #[tracing::instrument(skip(self))]
    pub async fn collection_from_name(
        &self,
        collection_name: &str,
    ) -> Result<Collection, ZoteroError> {
        let collections = self.get_collections(None).await?;
        let matching_collections: Vec<_> = collections
            .iter()
            .filter(|w| w.name.to_lowercase() == collection_name.to_lowercase())
            .collect();

        match matching_collections.len() {
            0 => Err(ZoteroError::CustomError(format!(
                "No collection with name {} found",
                collection_name
            ))),
            1 => Ok(matching_collections[0].clone()),
            _ => Err(ZoteroError::CustomError(format!(
                "Multiple collections with name {} found",
                collection_name
            ))),
        }
    }

    async fn process_batch(
        endpoint: String,
        offset: i32,
        limit: i32,
        client: &ZoteroClient,
    ) -> Result<Vec<Item>, ZoteroError> {
        let limit = limit.to_string();
        let offset = offset.to_string();
        let params = vec![("limit", limit.as_str()), ("start", offset.as_str())];
        let response = client.get(&endpoint, Some(params)).await?;
        let items_response = response.json::<Vec<ItemsResponse>>().await?;

        Ok(items_response.iter().map(|c| c.data.clone()).collect())
    }
}
