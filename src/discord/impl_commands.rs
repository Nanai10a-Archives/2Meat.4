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

#[serenity::async_trait]
impl MutCommand for RefWrap<DiscordTransceiver> {
    async fn mut_com(&self) -> anyhow::Result<()> {
        todo!()
    }
}

#[serenity::async_trait]
impl DropCommand for RefWrap<DiscordTransceiver> {
    async fn drop_com(&self) -> anyhow::Result<()> {
        let mut locked = self.lock().await;

        match *locked {
            None => Err(anyhow::Error::msg("not found (already deleted).")),
            Some(_) => {
                // FIXME: 削除時の処理の追加
                *locked = None;
                Ok(())
            }
        }
    }
}

#[serenity::async_trait]
impl SubscCommand<RefWrap<DiscordTransceiver>> for RefWrap<DiscordTransceiver> {
    async fn subsc_com(&self, target: &RefWrap<DiscordTransceiver>) -> anyhow::Result<()> {
        let mut self_locked = self.lock().await;
        let self_ref = match self_locked.as_mut() {
            None => todo!(),
            Some(item) => item,
        };

        let mut target_locked = target.lock().await;
        let target_ref = match target_locked.as_mut() {
            None => todo!(),
            Some(item) => item,
        };

        self_ref
            .subscribe_push(target_ref.get_id(), target_ref.get_subscriber())
            .unwrap();

        Ok(())
    }
}

#[serenity::async_trait]
impl ExitCommand<RefWrap<DiscordTransceiver>> for RefWrap<DiscordTransceiver> {
    async fn exit_com(&self, target: &RefWrap<DiscordTransceiver>) -> anyhow::Result<()> {
        let mut self_locked = self.lock().await;
        let self_ref = match self_locked.as_mut() {
            None => todo!(),
            Some(item) => item,
        };

        let target_locked = target.lock().await;
        let target_ref = match target_locked.as_ref() {
            None => todo!(),
            Some(item) => item,
        };

        self_ref.subscribe_remove(target_ref.get_id()).unwrap();

        Ok(())
    }
}
