use std::fmt::{Display, Formatter, Write};

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

impl Display for FormattedData {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        String::new().write_fmt(format_args!(
            "{}\
            \
            {}\
            \
            at {}",
            self.author, self.content, self.timestamp
        ))
    }
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

impl Display for Author {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.nickname {
            None => String::new().write_fmt(format_args!(
                "{}: {} on {:?}",
                self.name, self.id, self.place
            )),
            Some(nick) => String::new().write_fmt(format_args!(
                "{} ({}): {} on {:?}",
                nick, self.name, self.id, self.place
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdditionalContent {
    // TODO: なんか情報を付け足したい場合はここで 依存がおかしくならない程度に
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Place {
    Discord { channel_id: u64 },
    Twitter,
    Mastodon,
    Unknown,
    // TODO: Frontendが増える度に更新
}
