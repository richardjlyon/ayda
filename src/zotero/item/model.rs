use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemsResponse {
    pub data: Item,
}

/// An item in the Zotero library
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub key: String,
    pub version: i64,
    #[serde(rename = "parentItem")]
    pub parent_item: Option<String>,
    #[serde(rename = "itemType")]
    pub item_type: ItemType,
    pub title: String,
    pub creators: Option<Vec<Creator>>,
    #[serde(rename = "abstractNote")]
    pub abstract_note: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub filename: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "dateAdded", deserialize_with = "deserialize_utc_date")]
    pub date_added: DateTime<Utc>,
    #[serde(rename = "dateModified", deserialize_with = "deserialize_utc_date")]
    pub date_modified: DateTime<Utc>,
}

impl Item {
    pub fn is_pdf(&self) -> bool {
        self.content_type.as_deref() == Some("application/pdf")
    }

    pub fn filepath(&self, root: &std::path::Path) -> Option<PathBuf> {
        self.filename
            .as_ref()
            .filter(|_| self.is_pdf())
            .map(|name| root.join(&self.key).join(name))
    }
}

/// The type of the item
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum ItemType {
    Artwork,
    Attachment,
    AudioRecording,
    Bill,
    BlogPost,
    Book,
    BookSection,
    Case,
    ConferencePaper,
    Dataset,
    DictionaryEntry,
    Document,
    Email,
    EncyclopediaArticle,
    Film,
    ForumPost,
    Hearing,
    InstantMessage,
    Interview,
    JournalArticle,
    Letter,
    MagazineArticle,
    Manuscript,
    Map,
    NewspaperArticle,
    Note,
    Patent,
    Podcast,
    Preprint,
    Presentation,
    RadioBroadcast,
    Report,
    #[serde(rename = "computerProgram")]
    Software,
    Standard,
    Statute,
    #[serde(rename = "tvBroadcast")]
    TvBroadcast,
    Thesis,
    #[serde(rename = "videoRecording")]
    VideoRecording,
    #[serde(rename = "webpage")]
    WebPage,
}

/// A creator of the item
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Creator {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "creatorType")]
    pub creator_type: String,
}

/// A tag of the item
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tag {
    pub tag: String,
    // #[serde(rename = "type")]
    // pub tag_type: Option<i16>,
}

/// A struct to represent information to update an item with
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ItemUpdateData {
    #[serde(rename = "abstractNote", skip_serializing_if = "Option::is_none")]
    pub abstract_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<Creator>>,
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
