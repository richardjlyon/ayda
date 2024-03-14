//! Zotero API '/items' endpoints

use futures::StreamExt;

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

        // FIXME: items is limited to 100 items by default

        // Items returns 25 items by default, so we set the limit to 999 to get all items
        let params = match params {
            Some(mut p) => {
                p.extend(vec![("limit", "999")]);
                Some(p)
            }
            None => Some(vec![("limit", "999")]),
        };

        let response = self.get("items", params).await?;
        let headers = response.headers().clone();
        // print total results
        let total_results: i32 = headers.get("Total-Results").unwrap().to_str().unwrap().parse().unwrap();

        println!("Total results: {}", total_results);

        let mut items = Vec::new();
        let mut offset = 1300; //FIXME - revert to 0
        while offset < total_results {
            println!("Offset: {}", offset);
            let offset_string = offset.to_string(); // create a new variable for the offset string
            let params = vec![("limit", "100"), ("start", &offset_string)]; // use the new variable here
            let response = self.get("items", Some(params)).await?;
            let items_response = response.json::<Vec<ItemsResponse>>().await?;
            items.extend(items_response.iter().map(|c| c.data.clone()));
            offset += 100;
        }

        // let items = fetch_items_in_batches(total_results).await?;

        // spawn threads to get all items in batches of 100 items to the total results
        // let mut items = Vec::new();
        // let mut offset = 600; //FIXME - revert to 0
        // while offset < total_results {
        //     println!("Offset: {}", offset);
        //     let offset_string = offset.to_string(); // create a new variable for the offset string
        //     let params = vec![("limit", "100"), ("start", &offset_string)]; // use the new variable here
        //     let response = self.get("items", Some(params)).await?;
        //     let items_response = response.json::<Vec<ItemsResponse>>().await?;
        //     items.extend(items_response.iter().map(|c| c.data.clone()));
        //     offset += 100;
        // }

        Ok(items)
    }

    // async fn process_batch(offset: usize, limit: usize, client: &Self) -> Result<Vec<ItemsResponse>, ZoteroError> {
    //     let offset_string = offset.to_string();
    //     let params = vec![("limit", "100"), ("start", &offset_string)];
    //     let response = client.get("items", Some(params)).await?;
    //     let items_response = response.json::<Vec<ItemsResponse>>().await?;
    //     Ok(items_response.iter().map(|c| c.data.clone()).collect())
    // }


    /// GET /items/{itemKey}
    /// A specific item in the library
    pub async fn get_items_item_key(
        &self,
        item_key: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Item, ZoteroError> {
        let response = self
            .get_deserialized::<ItemsResponse>(&format!("items/{}", item_key), params)
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
            .get_deserialized::<Vec<ItemsResponse>>(&format!("collections/{}/items", collection_key), params)
            .await?;

        let items = response.iter().map(|c| c.data.clone()).collect();

        Ok(items)

        // Ok(res.into_iter().filter(|x| x.is_pdf()).collect())
    }
}

pub fn fetch_items_in_batches(total_results: i32, client: &ZoteroClient) -> impl futures::stream::Stream<Item=Item> + '_ {
    const CHUNK_SIZE: i32 = 100;
    let chunks = total_results / CHUNK_SIZE; // TODO: maybe add one

    let futs = futures::stream::FuturesUnordered::new();
    for x in 0..chunks {
        futs.push(process_batch(x * CHUNK_SIZE, CHUNK_SIZE, client));
    }

    // 1. vec <- first page to resolve
    // 2. vec <- 2nd page to resolve
    // 3. vec
    let futs = futs.filter_map(|f| async { f.ok() });

    // 1. vec[0][0]
    // 2. vec[0][1]
    let futs = futs.flat_map(futures::stream::iter);

    futs
}

// fn process_batch(offset: i32, limit: i32, client: &ZoteroClient) -> impl Future<Output = Result<Vec<Item>, ZoteroError>> {
async fn process_batch(offset: i32, limit: i32, client: &ZoteroClient) -> Result<Vec<Item>, ZoteroError> {
    let limit = limit.to_string();
    let offset = offset.to_string();
    let params = vec![("limit", limit.as_str()), ("start", offset.as_str())];
    let response = client.get("items", Some(params)).await?;
    let items_response = response.json::<Vec<ItemsResponse>>().await?;

    Ok(items_response.iter().map(|c| c.data.clone()).collect())
}