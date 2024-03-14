mod common;

mod tests {
    use zot2llm::zotero::client::ZoteroClient;

    use crate::common::ZoteroFixture;

// // Construction ///////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_client_new() {
        let client = ZoteroClient::new("key", "user");
        assert_eq!(client.base_url, "https://api.zotero.org/users/user");
    }

    // Collections ///////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_collections() {
        let fixture = ZoteroFixture::new().await;
        let collections = fixture.client.get_collections(None).await.unwrap();

        assert!(collections.len() > 0);
    }

    #[tokio::test]
    async fn test_get_collections_collection_key() {
        let fixture = ZoteroFixture::new().await;
        let collections = fixture.client.get_collections(None).await.unwrap();
        let collection_key = collections[0].key.clone();
        let collection = fixture
            .client
            .get_collections_collection_key(&collection_key, None)
            .await
            .unwrap();

        assert!(collection.key == collection_key);
    }

    #[tokio::test]
    async fn test_get_collections_collection_key_items() {
        let fixture = ZoteroFixture::new().await;
        let collections = fixture.client.get_collections(None).await.unwrap();
        let collection_key = collections[3].key.clone();
        let params = None;
        let items = fixture
            .client
            .get_collections_collection_key_items(&collection_key, params)
            .await
            .unwrap();

        assert!(items.len() > 0);
    }

    #[tokio::test]
    async fn test_get_collections_collection_key_items_filtered() {
        let fixture = ZoteroFixture::new().await;
        let collections = fixture.client.get_collections(None).await.unwrap();
        let collection_key = collections[3].key.clone();
        let params_array = [
            ("itemType", "attachment"),
            ("format", "json"),
            ("linkMode", "imported_file"),
        ];
        let params = Some(params_array.iter().map(|(k, v)| (*k, *v)).collect());
        let items = fixture
            .client
            .get_collections_collection_key_items(&collection_key, params)
            .await
            .unwrap();

        assert!(items.len() > 0);
    }

    #[tokio::test]
    async fn test_get_collections_top() {}

    #[tokio::test]
    async fn test_get_collections_collection_key_collections() {}

    // Items /////////////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_items() {
        let fixture = ZoteroFixture::new().await;
        let items = fixture.client.get_items(None).await.unwrap();
        assert!(items.len() > 0);

        dbg!(items.len());
    }

    #[tokio::test]
    async fn test_get_items_batched() {
        let fixture = ZoteroFixture::new().await;
        let items_stream = zot2llm::zotero::api::endpoints::items::fetch_items_in_batches(2000, &fixture.client);
        use futures::StreamExt;
        let data: Vec<_> = items_stream.collect().await;
        println!("{}", data.len());
    }

    #[tokio::test]
    async fn test_get_items_item_key() {
        let fixture = ZoteroFixture::new().await;
        let items = fixture.client.get_items(None).await.unwrap();
        let item_key = items[0].key.clone();
        let item = fixture
            .client
            .get_items_item_key(&item_key, None)
            .await
            .unwrap();
        assert!(item.key == item_key);
    }

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
