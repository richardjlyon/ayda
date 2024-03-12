#[derive(Debug, serde::Deserialize)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub title: String,
    pub cached: bool,
}

impl From<&Item> for Document {
    fn from(item: &Item) -> Self {
        Document {
            id: item.id.clone().unwrap(),
            name: item.name.clone(),
            title: item.title.clone().unwrap(),
            cached: item.cached.unwrap(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Item {
    pub items: Option<Vec<Item>>,
    #[serde(rename = "type")]
    pub doc_type: String,
    pub name: String,
    pub title: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "docAuthor")]
    pub doc_author: Option<String>,
    pub cached: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DocumentsResponse {
    #[serde(rename = "localFiles")]
    pub local_files: Item,
}
