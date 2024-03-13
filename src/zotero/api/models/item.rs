use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

// use serde_with::DateTimeWithTimeZone;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemsResponse {
    pub data: Item,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub key: String,
    pub filename: Option<String>,
    pub title: String,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "dateAdded", deserialize_with = "deserialize_utc_date")]
    pub date_added: DateTime<Utc>,
}

impl ItemsResponse {
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

fn deserialize_utc_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    match DateTime::parse_from_rfc3339(&date_str) {
        Ok(datetime) => Ok(datetime.into()),
        Err(_) => Err(serde::de::Error::custom("Invalid date format")),
    }
}
