use ayda::anythingllm::client::AnythingLLMClient;
use ayda::anythingllm::workspace::Workspace;
use ayda::zotero::client::ZoteroClient;
use ayda::Config;

pub struct AnythingLLMFixture {
    pub client: AnythingLLMClient,
}

impl AnythingLLMFixture {
    pub fn new() -> Self {
        let config = Config::from_file().unwrap();
        let client = AnythingLLMClient::new(
            &config.anythingllm_ip,
            &config.anythingllm_port,
            &config.anythingllm_api_key,
        );

        Self { client }
    }

    pub async fn with_fixture<F, Fut>(mut self, func: F)
    where
        F: FnOnce(Workspace, AnythingLLMClient) -> Fut,
        Fut: Future<Output = ()>,
    {
        // Create a test workspace
        let uuid = uuid::Uuid::new_v4();
        let workspace_name = format!("DELETE ME {}", uuid);

        let workspace = self.client.create_workspace(&workspace_name).await.unwrap();
        let slug = workspace.slug.clone();
        func(workspace, self.client.clone()).await;

        if let Err(e) = self.client.delete_workspace_slug(&slug).await {
            tracing::error!("unable to delete {}: {}", workspace_name, e);
        }
    }
}

use std::future::Future;

pub struct ZoteroFixture {
    pub client: ZoteroClient,
}

impl ZoteroFixture {
    pub async fn new() -> Self {
        let config = Config::from_file().unwrap();
        let client = ZoteroClient::new(&config.zotero_api_key, &config.zotero_user_id);

        Self { client }
    }
}
