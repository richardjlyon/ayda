use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemData {
    pub key: String,
    pub filename: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub data: ItemData,
}

impl Item {
    pub fn is_pdf(&self) -> bool {
        self.data.content_type.as_deref() == Some("application/pdf")
    }

    pub fn filepath(&self, root: &str) -> Option<String> {
        self.is_pdf().then(|| {
            format!(
                "{}/{}/{}",
                root,
                self.data.key,
                self.data.filename.as_ref().unwrap()
            )
        })
    }
}
