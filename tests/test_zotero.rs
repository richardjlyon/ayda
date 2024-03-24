mod common;

mod tests {
    use ayda::zotero::client::ZoteroClient;
    use ayda::zotero::item::model::{Item, ItemUpdateData, Tag};
    use color_eyre::owo_colors::AnsiColors::Default;
    use std::default::Default as stdDefault;

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
    async fn test_collection_from_name_is_valid() {
        let fixture = ZoteroFixture::new().await;
        let collection_name = "covid";
        let collection = fixture
            .client
            .collection_from_name(collection_name)
            .await
            .unwrap();

        assert!(collection.name.to_lowercase() == collection_name.to_lowercase());
    }

    #[tokio::test]
    async fn test_collection_from_name_is_invalid() {
        let fixture = ZoteroFixture::new().await;
        let collection_name = "invalid collection name";
        let collection = fixture.client.collection_from_name(collection_name).await;

        assert!(collection.is_err());
    }

    #[tokio::test]
    async fn test_get_collections_top() {}

    #[tokio::test]
    async fn test_get_collections_collection_key_collections() {}

    // Items /////////////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_get_items() {
        use futures::StreamExt;
        let fixture = ZoteroFixture::new().await;
        let items: Vec<Item> = fixture.client.get_items().collect().await;
        assert!(items.len() > 0);

        dbg!(&items[0]);
    }

    // getting the parent of an item with a parent should return the parent item

    #[tokio::test]
    // #[ignore] // This operates on a live Zotero library
    async fn test_get_item_parent_some() {
        let fixture = ZoteroFixture::new().await;
        let berger_key = "DVUR4DH8";
        let attachment_item = fixture.client.get_items_item_key(berger_key).await.unwrap();
        let parent_item = fixture
            .client
            .get_item_parent(&attachment_item)
            .await
            .unwrap();

        assert_eq!(parent_item.clone().unwrap().key, "NKXWCXKP".to_string());

        dbg!(&parent_item);
    }

    // getting the parent of an item without a parent should return None

    #[tokio::test]
    #[ignore] // This operates on a live Zotero library
    async fn test_get_item_parent_none() {
        let fixture = ZoteroFixture::new().await;
        let item_key = "NKXWCXKP"; // has no parent
        let item = fixture.client.get_items_item_key(item_key).await.unwrap();
        let parent_item = fixture.client.get_item_parent(&item).await.unwrap();

        assert!(parent_item.is_none());
    }

    // changing the parent of an item should update the parent item

    #[tokio::test]
    // #[ignore] // This operates on a live Zotero library
    async fn test_change_parent_item() {
        use serde_json::json;

        let fixture = ZoteroFixture::new().await;
        let item_key = "DVUR4DH8";
        let item = fixture.client.get_items_item_key(&item_key).await.unwrap();

        let data = ItemUpdateData {
            abstract_note: Some("TEST DELETE ME".to_string()),
            ..stdDefault::default()
        };

        let result = fixture.client.change_parent_item(&item, &data).await;

        assert!(result.is_ok());
    }

    // Items Batched //////////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    #[ignore] // This will retrieve all items in the library, and takes a while
    async fn test_get_items_batched() {
        use futures::StreamExt;
        let fixture = ZoteroFixture::new().await;
        let items_stream = fixture.client.get_items();
        let data: Vec<_> = items_stream.collect().await;

        assert!(data.len() > 0);
    }

    #[tokio::test]
    #[ignore] // This requires a collection with items
    async fn test_get_collections_collection_key_items_batched() {
        use futures::StreamExt;
        let fixture = ZoteroFixture::new().await;
        let collections = fixture.client.get_collections(None).await.unwrap();
        let collection_key = collections[0].key.clone();

        // let params = None;

        let items_stream = fixture
            .client
            .get_collections_collection_key_items_batched(collection_key);

        let data: Vec<_> = items_stream.collect().await;
        println!("Fetched {} items", data.len());

        assert!(data.len() > 0);
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
