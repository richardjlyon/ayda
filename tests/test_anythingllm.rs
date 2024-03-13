mod common;

mod tests {
    use std::env;

    use zot2llm::anythingllm::client::AnythingLLMClient;
    use zot2llm::anythingllm::error::LLMError;

    use crate::common::AnythingLLMFixture;

    //  // Construction ///////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_client_new() {
        dotenv::dotenv().ok();
        let api_key = "api_key";
        let ip = "10.13.10.8";
        let port = "3001";
        let client = AnythingLLMClient::new(ip, port, api_key);

        assert_eq!(client.base_url, "http://10.13.10.8:3001/api/v1");
    }

    // Authentication /////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_auth_ok() {
        dotenv::dotenv().ok();
        let api_key = &env::var("ANYTHINGLLM_API_KEY").expect("API key not found");
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = AnythingLLMClient::new(ip, port, api_key);

        assert!(client.get_auth().await.is_ok());
    }

    #[tokio::test]
    async fn test_get_auth_err() {
        dotenv::dotenv().ok();
        let api_key = "INVALID_API_KEY";
        let ip = &env::var("ANYTHINGLLM_IP").expect("IP not found");
        let port = &env::var("ANYTHINGLLM_PORT").expect("port not found");
        let client = AnythingLLMClient::new(ip, port, api_key);

        assert!(client.get_auth().await.is_err());
    }

    // Workspace //////////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_post_workspace_new() {
        let fixture = AnythingLLMFixture::new().await;
        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_workspaces() {
        let fixture = AnythingLLMFixture::new().await;
        let workspaces = fixture.client.get_workspaces().await.unwrap();
        let workspace_slug = &fixture.workspace.slug;

        assert!(workspaces.len() > 0);
        assert!(workspaces
            .iter()
            .any(|w| w.slug == workspace_slug.to_string()));

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_workspace_slug() {
        let fixture = AnythingLLMFixture::new().await;
        let test_workspace_slug = &fixture.workspace.slug;
        let workspace = fixture
            .client
            .get_workspace_slug(&test_workspace_slug)
            .await
            .unwrap();

        assert_eq!(workspace.slug, test_workspace_slug.to_string());

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_workspace_invalid_slug() {
        let fixture = AnythingLLMFixture::new().await;
        let workspace = fixture
            .client
            .get_workspace_slug("invalid-workspace-slug")
            .await;

        println!("{:?}", workspace);

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_delete_workspace_slug() {
        let fixture = AnythingLLMFixture::new().await;
        let test_workspace_slug = &fixture.workspace.slug;
        let _ = fixture
            .client
            .delete_workspace_slug(&test_workspace_slug)
            .await
            .unwrap();
        let workspaces = fixture.client.get_workspaces().await.unwrap();

        assert!(!workspaces
            .iter()
            .any(|w| w.slug == test_workspace_slug.to_string()));
    }

    #[tokio::test]
    async fn test_delete_workspace_invalid_slug() {
        let fixture = AnythingLLMFixture::new().await;
        let response = fixture
            .client
            .delete_workspace_slug("invalid-workspace-slug")
            .await;

        match response {
            Ok(_) => panic!("Expected an error, but got Ok(_)"),
            Err(err) => match err {
                LLMError::BadRequest(_) => (), // Test passes if we get here
                _ => panic!("Expected LLMError::BadRequest, but got a different error"),
            },
        }

        fixture.client.get_workspaces().await.unwrap();
    }

    #[tokio::test]
    async fn test_workspace_slug_update_embeddings() {}

    // Document tests /////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_documents() {
        let fixture = AnythingLLMFixture::new().await;

        let docs = fixture.client.get_documents().await.unwrap();
        assert!(docs.len() > 0);

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_post_document_upload() {
        let fixture = AnythingLLMFixture::new().await;
        let file_path = "tests/test_data/2022-01-01-Test-Document.pdf";
        let doc = fixture
            .client
            .post_document_upload(&file_path)
            .await
            .unwrap();

        // confirm the document is in the workspace

        fixture.remove().await;
    }

    #[tokio::test]
    async fn test_post_document_upload_invalid() {
        // let fixture = Fixture::new().await;
        //
        // // upload a test document
        // let doc = fixture.client.post_document_upload().await.unwrap();
        // // confirm the document is in the workspace
        // // delete the workspace
        //
        // fixture.remove().await;
    }

    #[tokio::test]
    async fn test_get_document_docname() {}

    #[tokio::test]
    async fn test_get_document_invalid_docname() {}
}
