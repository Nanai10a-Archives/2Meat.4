use crate::interface::discord::DiscordInterface;
use crate::model::data::FormattedData;
use serenity::model::prelude::Message;
use std::sync::Arc;
use uuid::Uuid;

pub mod discord;

pub enum InterfaceRecvArg {
    Discord(Message),
}

// TODO: Transceiverを新規作成する時, どのように参照を渡すのか
pub enum Interface {
    Discord(Arc<DiscordInterface>),
}

impl Interface {
    pub async fn send(&self, tr_id: Uuid, data: FormattedData) -> anyhow::Result<()> {
        match self {
            Interface::Discord(interface) => interface.send(tr_id, data).await,
        }
    }
}
