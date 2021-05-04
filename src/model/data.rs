use std::fmt::{Display, Formatter, Write};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedData {
    pub content: String,
    pub attachments: SmallVec<[Attachments; 4]>,
    pub author: Author,
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
    pub data: Option<Box<Path>>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Place {
    Discord { channel_id: u64 },
    Twitter,
    Mastodon, // TODO: server domainを書けるかも？
    // Direct { protocol: Protocol, ip: IpAddr }, // TODO: 要検討
    Unknown,
    // TODO: Frontendが増える度に更新
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Protocol {
    Http,
    WebSocket,
}
