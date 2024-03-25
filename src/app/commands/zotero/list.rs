use itertools::Itertools;

use crate::app;
use crate::app::commands;
use crate::zotero::collection::model::Collection;

/// List all Zotero collections.
///
pub async fn list_collections() -> eyre::Result<()> {
    let client = commands::zotero_client();
    let collections = client.get_collections(None).await?;
    let column_titles = vec!["COLLECTION NAME"];
    let data = data_from_collections(collections);
    app::display_table(column_titles, data);

    Ok(())
}

// Extract data, sort alphabetically, convert to lowercase, and return as a Vec<Vec<String>>.
fn data_from_collections(collections: Vec<Collection>) -> Vec<Vec<String>> {
    let data: Vec<Vec<String>> = collections
        .iter()
        .map(|c| vec![c.name.to_lowercase().clone()])
        .sorted_by(|a, b| a[0].cmp(&b[0]))
        .collect();

    data
}
