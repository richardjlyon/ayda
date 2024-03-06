use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionData {
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub data: CollectionData,
}
