use std::env;

use zot2llm::anythingllm::client::AnythingLLMClient;
use zot2llm::anythingllm::workspace::Workspace;

pub struct Fixture {
    pub client: AnythingLLMClient,
    pub workspace: Workspace,
}

impl Fixture {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();
        let api_key = &env::var("ANYTHINGLLM_API_KEY").expect("API key not found");
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = AnythingLLMClient::new(ip, port, api_key);

        // Create a test workspace
        let uuid = uuid::Uuid::new_v4();
        let workspace_name = format!("DELETE ME {}", uuid);
        let workspace = client.post_workspace_new(&workspace_name).await.unwrap();

        Self { client, workspace }
    }

    pub async fn remove(self) {
        let _ = &self
            .client
            .delete_workspace_slug(&self.workspace.slug)
            .await;
    }
}
