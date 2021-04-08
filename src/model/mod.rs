use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedData {
    pub content: String,
    pub attachments: Vec<Attachments>,
    pub author: Author,
    pub additional_contents: Option<Vec<AdditionalContent>>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachments {
    pub a_type: AttachmentType,
    pub url: String,
    pub binary: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttachmentType {
    Image,
    Audio,
    Movie,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub nickname: Option<String>,
    pub id: String,
    pub place: Place,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdditionalContent {
    DiscordPlaceInfo,
    // TODO: なんか情報を付け足したい場合はここで 依存がおかしくならない程度に
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Place {
    Discord,
    Twitter,
    Mastodon,
    Unknown,
    // TODO: Frontendが増える度に更新
}

#[derive(Debug)]
pub enum Error {
    Serenity(serenity::Error),
}