mod common;

mod tests {
    use std::path::PathBuf;

    use ayda::anythingllm::client::AnythingLLMClient;
    use ayda::Config;

    use crate::common::AnythingLLMFixture;

    //  // Construction ///////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_client_new() {
        let api_key = "api_key";
        let ip = "10.13.10.8";
        let port = "3001";
        let client = AnythingLLMClient::new(ip, port, api_key);

        assert_eq!(client.base_url_api_v1, "http://10.13.10.8:3001/api/v1");
    }

    // Authentication /////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_auth_ok() {
        let config = Config::from_file().unwrap();

        let client = AnythingLLMClient::new(
            &config.anythingllm_ip,
            &config.anythingllm_port,
            &config.anythingllm_api_key,
        );

        assert!(client.get_auth().await.is_ok());
    }

    #[tokio::test]
    async fn test_get_auth_err() {
        let config = Config::from_file().unwrap();
        let client = AnythingLLMClient::new(
            &config.anythingllm_ip,
            &config.anythingllm_port,
            "invalid_api_key",
        );

        assert!(client.get_auth().await.is_err());
    }

    // Document tests /////////////////////////////////////////////////////////////////////////////

    // #[tokio::test]
    // #[tracing_test::traced_test]
    // // #[ignore] // NOTE: This test is ignored because it requires a valid document to be uploaded
    // async fn test_get_documents() {
    //     let fixture = AnythingLLMFixture::new();
    //     let test_doc_filepath = PathBuf::from("tests/test_data/DELETE ME test document.pdf");
    //
    //     fixture.with_fixture(|w| async {
    //
    //     })
    //
    //     let doc = fixture
    //         .client
    //         .post_document_upload(&test_doc_filepath)
    //         .await
    //         .unwrap();
    //     // fixture.remove().await;
    //
    //     let docs = fixture.client.get_documents().await.unwrap();
    //     assert!(docs.len() > 0);
    // }
    //
    // #[tokio::test]
    // async fn test_get_document_slug() {
    //     let fixture = AnythingLLMFixture::new().await;
    //     let test_doc_filepath = PathBuf::from("tests/test_data/DELETE ME test document.pdf");
    //
    //     let doc = fixture
    //         .client
    //         .post_document_upload(&test_doc_filepath)
    //         .await
    //         .unwrap();
    //
    //     let workspace_slug = &fixture.workspace.slug;
    //     let docs = fixture.client.get_api_workspace_slug(workspace_slug).await;
    //
    //     dbg!(docs);
    //     // fixture.remove().await;
    //
    //     // assert_eq!(doc.slug, doc_slug.to_string());
    // }

    // #[tokio::test]
    // #[ignore] // NOTE: This test is ignored because it requires a valid document to be uploaded
    // async fn test_post_document_upload() {
    //     let fixture = AnythingLLMFixture::new().await;
    //     // let file_path = PathBuf::from("tests/test_data/DELETE ME test document.pdf");
    //     let ok_doc = "tests/test_data/DELETE ME test document.pdf";
    //     let problem_doc = "tests/test_data/Mckie and Macrae-legal issues and options arising in relation to the venue operator’s decision to cancel her appearanceparticipation, scheduled for 10 August 2023, at the Edinburgh Fringe show In Conversation with Jo.pdf";
    //     let problem_doc_renamed = "tests/test_data/PROBLEM_FILE.pdf";
    //     let file_path = PathBuf::from(problem_doc_renamed);
    //
    //     let doc = fixture
    //         .client
    //         .post_document_upload(&file_path)
    //         .await
    //         .unwrap();
    //
    //     // confirm the document is in the workspace
    //     // TODO: implement this (there isn't a pulic API endpoint for this yet)
    //
    //     fixture.remove().await;
    // }

    #[tokio::test]
    async fn test_delete_document() {
        let fixture = AnythingLLMFixture::new();
        let test_doc_filepath = PathBuf::from("tests/test_data/DELETE ME test document.pdf");

        fixture
            .with_fixture(|w, c| async move {
                let doc = c.post_document_upload(&test_doc_filepath).await.unwrap();

                let before_count = c.get_documents().await.unwrap().len();
                let docs = vec![doc.location.clone().unwrap()];
                let _ = c.delete_api_system_remove_documents(docs).await.unwrap();
                let after_count = c.get_documents().await.unwrap().len();
                assert_eq!((before_count - after_count), 1);
            })
            .await;
    }

    #[tokio::test]
    async fn test_post_document_upload_invalid() {}

    #[tokio::test]
    async fn test_get_document_docname() {}

    #[tokio::test]
    async fn test_get_document_invalid_docname() {}
}
