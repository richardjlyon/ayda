use dotenv::dotenv;
use std::env;
use zotero_llm::anythingllm::client::AnythingLLMClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = AnythingLLMClient::new(
        "10.13.10.8",
        3001,
        &env::var("ANYTHINGLLM_API_KEY").expect("API key not found"),
    );

    match client.documents().await {
        Ok(docs) => docs.iter().for_each(|doc| println!("{:?}", doc)),
        Err(e) => println!("Error: {}", e),
    }
}
