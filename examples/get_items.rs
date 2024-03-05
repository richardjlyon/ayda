use dotenv::dotenv;
use std::env;
use zotero_llm::zotero::client::ZoteroClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let collection_key = "NQF36WE7";
    let api_key = env::var("ZOTERO_API_KEY").expect("API key not found");
    let user_id = env::var("ZOTERO_USER_ID").expect("User ID not found");

    let z = ZoteroClient::new(&api_key, &user_id);

    match z
        .fetch_items_with_pdfs(&collection_key)
        .await
    {
        Ok(collections) => {
            for collection in collections {
                println!("{:?}", collection);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
