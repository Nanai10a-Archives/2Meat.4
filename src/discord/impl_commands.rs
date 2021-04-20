use crate::commands::*;
use crate::discord::transceiver::{
    DiscordTransceiver, DiscordTransceivers, Transceiver, Transceivers,
};
use crate::utils::RefWrap;
use anyhow::Error;
use serenity::prelude::Mutex;
use std::sync::Arc;

#[serenity::async_trait]
impl NewCommand<RefWrap<DiscordTransceivers>> for DiscordTransceiver {
    async fn new_com(parent: &RefWrap<DiscordTransceivers>) -> anyhow::Result<RefWrap<Self>> {
        let mut locked = parent.lock().await;

        let trcvs = match locked.as_mut() {
            None => todo!(),
            Some(item) => item,
        };

        match trcvs.new_child().await {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }
}
