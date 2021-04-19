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
        match {
            match (*parent.lock().await).as_mut() {
                None => todo!(),
                Some(item) => item,
            }
        }
        .new_child()
        .await
        {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }
}
