use chrono::{DateTime, Utc};

pub struct FormattedData {
    pub content: String,
    pub attachments: Vec<Attachments>,
    pub author: Author,
    pub additional_contents: Option<Vec<AdditionalContent>>,
    pub timestamp: DateTime<Utc>,
}

pub struct Attachments {
    pub a_type: AttachmentType,
    pub url: String,
    pub binary: Option<Vec<u8>>,
}

pub enum AttachmentType {
    Image,
    Audio,
    Movie,
    Other,
}

pub struct Author {
    pub name: String,
    pub nickname: Option<String>,
    pub id: String,
    pub place: Place,
}

pub enum AdditionalContent {
    DiscordPlaceInfo,
    // TODO: なんか情報を付け足したい場合はここで 依存がおかしくならない程度に
}

pub enum Place {
    Discord,
    Twitter,
    Mastodon,
    Unknown,
    // TODO: Frontendが増える度に更新
}
