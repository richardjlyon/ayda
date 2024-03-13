use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionsResponse {
    pub data: Collection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub key: String,
    pub name: String,
}
