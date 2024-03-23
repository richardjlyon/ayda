// anythingllm workspace integration tests

mod common;

mod tests {
    use crate::common::AnythingLLMFixture;
    use ayda::anythingllm::client::AnythingLLMClient;
    use ayda::app::commands::workspace::import::UpdateParameter;
    use ayda::Config;
    use std::path::PathBuf;

    // getting a list of workspaces returns a non-zero list of Workspace objects

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_get_workspaces() {
        let fixture = AnythingLLMFixture::new();
        fixture
            .with_fixture(|w, c| async move {
                let workspaces = c.get_workspaces().await.unwrap();
                let workspace_slug = &w.slug;

                assert!(workspaces.len() > 0);
                assert!(workspaces
                    .iter()
                    .any(|w| w.slug == workspace_slug.to_string()));
            })
            .await;
    }

    // creating a workspace increases the number of workspaces by one

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_create_workspace() {
        let config = Config::from_file().unwrap();
        let client = AnythingLLMClient::new(
            &config.anythingllm_ip,
            &config.anythingllm_port,
            &config.anythingllm_api_key,
        );
        let before_count = client.get_workspaces().await.unwrap().len();
        let fixture = AnythingLLMFixture::new();

        fixture
            .with_fixture(|w, c| async move {
                let after_count = c.get_workspaces().await.unwrap().len();
                // assert_eq!(after_count, before_count + 1);
            })
            .await;
    }

    // getting a workspace by its slug returns the correct workspace

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_get_workspace_by_slug() {
        let fixture = AnythingLLMFixture::new();
        fixture
            .with_fixture(|w, c| async move {
                let test_workspace_slug = &w.slug;
                let workspace = c.get_workspace_by_slug(&test_workspace_slug).await.unwrap();

                assert_eq!(workspace.slug, test_workspace_slug.to_string());
            })
            .await;
    }

    // getting a workspace by its slug should fail if the slug is invalid

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_get_workspace_by_invalid_slug() {
        let fixture = AnythingLLMFixture::new();
        fixture
            .with_fixture(|w, c| async move {
                let workspace = c.get_workspace_by_slug("invalid-workspace-slug").await;
                assert!(workspace.is_err());
            })
            .await;
    }

    // getting a workspace by its name returns the correct workspace

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_workspace_from_name_is_valid() {
        let fixture = AnythingLLMFixture::new();
        fixture
            .with_fixture(|w, c| async move {
                let workspace = c.get_workspace_by_name(&w.name).await.unwrap();
                dbg!(&workspace);
                assert_eq!(workspace.id, w.id);
            })
            .await;
    }

    // getting a workspace by its name should fail if the name is invalid

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_workspace_from_name_is_invalid() {
        let fixture = AnythingLLMFixture::new();
        fixture
            .with_fixture(|w, c| async move {
                let workspace = c.get_workspace_by_name("Invalid workspace name").await;
                assert!(workspace.is_err());
            })
            .await;
    }

    // embedding documents in a workspace should return a non-zero list of Document objects
    // NOTE: this test requires and assumes document uploading works

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_update_embeddings() {
        // get a workspace fixture
        let fixture = AnythingLLMFixture::new();

        fixture
            .with_fixture(|w, c| async move {
                // add a document to the file system
                // NOTE: this does not embed the document in the workspace
                let test_doc_filepath =
                    PathBuf::from("tests/test_data/DELETE ME test document.pdf");
                let doc_before_count = c.get_documents().await.unwrap().len();
                let doc = c.post_document_upload(&test_doc_filepath).await.unwrap();

                let doc_after_count = c.get_documents().await.unwrap().len();
                assert_eq!(doc_after_count, doc_before_count + 1);

                // embed the document in the workspace
                // NOTE: this transfers the document from the file system to the workspace only
                let test_workspace_slug = &w.slug;
                let docs = vec![doc.clone().location.unwrap()];
                let _ = c
                    .update_embeddings(&test_workspace_slug, docs, UpdateParameter::Adds)
                    .await
                    .unwrap();

                let doc_after_count = c.get_documents().await.unwrap().len();
                assert_eq!(doc_after_count, doc_before_count + 1);

                // verify the document is in the workspace
                let documents = c
                    .get_workspace_by_slug(&test_workspace_slug)
                    .await
                    .unwrap()
                    .documents
                    .unwrap();

                assert_eq!(documents.len(), 1);
                assert_eq!(doc.clone().location.unwrap(), documents[0].docpath);

                // cleanup
                // NOTE: this test requires and assumes document deletion works

                let doc_vec = vec![doc.clone().location.unwrap()];
                let _ = c.delete_api_system_remove_documents(doc_vec).await.unwrap();
            })
            .await;
    }

    // deleting a workspace should remove the workspace from the list of workspaces
    // and all embedded documents in the workspace

    #[tokio::test]
    async fn test_delete_workspace_slug() {
        let fixture = AnythingLLMFixture::new();
        fixture
            .with_fixture(|w, c| async move {
                // add a document to the file system
                let test_doc_filepath =
                    PathBuf::from("tests/test_data/DELETE ME test document.pdf");
                let doc_before_count = c.get_documents().await.unwrap().len();
                let doc = c.post_document_upload(&test_doc_filepath).await.unwrap();

                // embed the document in the workspace
                let test_workspace_slug = &w.slug;
                let docs = vec![doc.clone().location.unwrap()];
                let _ = c
                    .update_embeddings(&test_workspace_slug, docs, UpdateParameter::Adds)
                    .await
                    .unwrap();

                // delete the workspace
                let workspace_slug = &w.slug;
                let _ = c.delete_workspace_slug(&workspace_slug).await.unwrap();
            })
            .await;
    }

    // deleting a workspace should fail if its name is invalid

    #[tokio::test]
    async fn test_delete_workspace_with_invalid_slug() {
        // get a workspace fixture
        let fixture = AnythingLLMFixture::new();
        fixture
            .with_fixture(|w, c| async move {
                let result = c.delete_workspace_slug("invalid slug").await;

                assert!(result.is_err());
            })
            .await;
    }
}
