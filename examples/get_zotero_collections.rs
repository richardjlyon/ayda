use std::env;

use dotenv::dotenv;

use zot2llm::zotero::client::ZoteroClient;
use zot2llm::zotero::models::collection::CollectionResponse;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let z = ZoteroClient::new(
        &env::var("ZOTERO_API_KEY").expect("API key not found"),
        &env::var("ZOTERO_USER_ID").expect("User ID not found"),
    );

    match z.get::<Vec<CollectionResponse>>("collections").await {
        Ok(collections) => collections
            .iter()
            .for_each(|collection| println!("{:?}", collection)),
        Err(e) => println!("Error: {}", e),
    }
}
