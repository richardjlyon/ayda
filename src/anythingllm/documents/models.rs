#[derive(Debug, serde::Deserialize)]
pub struct Document {
    pub id: String,
    pub name: Option<String>,
    pub title: Option<String>,
    pub cached: Option<bool>,
    pub location: Option<String>,
}

impl From<&Item> for Document {
    fn from(item: &Item) -> Self {
        Document {
            id: item.id.clone().unwrap(),
            name: item.name.clone(),
            title: item.title.clone(),
            cached: item.cached,
            location: item.location.clone(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct DocumentsResponse {
    #[serde(rename = "localFiles")]
    pub local_files: Item,
}

#[derive(Debug, serde::Deserialize)]
pub struct DocumentMultipartResponse {
    pub documents: Vec<Item>,
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Item {
    pub id: Option<String>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub items: Option<Vec<Item>>,
    #[serde(rename = "type")]
    pub doc_type: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "docAuthor")]
    pub doc_author: Option<String>,
    pub cached: Option<bool>,
}
