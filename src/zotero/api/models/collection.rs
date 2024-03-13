use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionResponse {
    pub data: CollectionResponseData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionResponseData {
    pub key: String,
    pub name: String,
}
