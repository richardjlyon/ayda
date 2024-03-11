use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

// v1/workspaces
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspacesResponse {
    pub workspaces: Vec<Workspace>,
}

#[derive(Deserialize, Debug)]
pub struct WorkspaceNewResponse {
    pub message: Option<String>,
    pub workspace: Workspace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    pub id: u8,
    pub name: String,
    pub slug: String,
    #[serde(rename = "createdAt", with = "date_format")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "lastUpdatedAt", with = "date_format")]
    pub last_updated_at: DateTime<FixedOffset>,
}

mod date_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Serializer};

    const FORMAT: &str = "%+";

    // Existing deserialize function
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }

    // New serialize function
    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }
}
