use dotenv::dotenv;
use std::env;
use zot2llm::zotero::client::ZoteroClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let z = ZoteroClient::new(
        &env::var("ZOTERO_API_KEY").expect("API key not found"),
        &env::var("ZOTERO_USER_ID").expect("User ID not found"),
    );

    match z.items_with_pdfs("NQF36WE7").await {
        Ok(items) => items
            .iter()
            .filter_map(|item| {
                item.filepath(
                    &env::var("ZOTERO_LIBRARY_ROOT_PATH").expect("Library root path not found"),
                )
            })
            .for_each(|path| println!("{:?}", path)),
        Err(e) => println!("Error: {}", e),
    }
}
