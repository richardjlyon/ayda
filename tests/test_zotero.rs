mod common;

mod tests {
    use zot2llm::zotero::client::ZoteroClient;

    // // Construction ///////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_client_new() {
        let client = ZoteroClient::new("key", "user");
        assert_eq!(client.base_url, "https://api.zotero.org/users/user");
    }

    // Collections ///////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_collections() {}

    #[tokio::test]
    async fn test_get_collections_collection_key() {}

    #[tokio::test]
    async fn test_get_collections_collection_key_items() {}

    #[tokio::test]
    async fn test_get_collections_top() {}

    #[tokio::test]
    async fn test_get_collections_collection_key_collections() {}

    // Items /////////////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_items() {}

    #[tokio::test]
    async fn test_get_items_item_key() {}

    // Item Types /////////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_item_types() {
        // let client = ZoteroClient::new("key", "user");
        // let item_types = client.get_item_types().await.unwrap();
        // assert!(item_types.len() > 0);
    }

    #[tokio::test]
    async fn test_get_item_fields() {}

    #[tokio::test]
    async fn test_get_valid_field_for_item_type() {}
}
