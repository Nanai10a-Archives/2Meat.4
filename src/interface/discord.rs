use std::ops::DerefMut;
use std::sync::Arc;

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::ClientBuilder;
use serenity::model::prelude::{ChannelId, Message, Ready};
use serenity::prelude::{Context, EventHandler};
use serenity::{Client, Error};
use smallvec::smallvec;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::model::data::{Author, FormattedData, Place};
use crate::transceiver::Transceivers;

pub struct DiscordInterface {
    serenity_ctx: RwLock<Context>,
    transceivers: Arc<Transceivers>,
    channel_to_transceiver_bindings: RwLock<Vec<(Uuid, u64)>>,
}

struct DiscordInterfaceWrapper(Arc<DiscordInterface>);

impl DiscordInterface {
    pub async fn new(
        token: impl AsRef<str>,
        transceivers: Arc<Transceivers>,
    ) -> anyhow::Result<Arc<Self>> {
        let arc = Arc::new(DiscordInterface {
            serenity_ctx: RwLock::new(unsafe {
                #[allow(deprecated, invalid_value)]
                std::mem::uninitialized()
            }),
            transceivers,
            channel_to_transceiver_bindings: RwLock::new(Vec::new()),
        });

        let intents = GatewayIntents::privileged();
        // TODO: Intentsの設定

        let res = ClientBuilder::new(token)
            .intents(intents)
            .event_handler(DiscordInterfaceWrapper(arc.clone()))
            .await;

        let mut serenity = match res {
            Ok(client) => client,
            Err(err) => return Err(anyhow::Error::new(err)),
        };

        tokio::spawn(async move {
            match serenity.start_autosharded().await {
                _ => {
                    // TODO: Resultが返ったときの処理
                    unimplemented!()
                }
            }
        });

        Ok(arc)
    }

    pub async fn receive(&self, receive_data: Message) -> anyhow::Result<()> {
        let data = FormattedData {
            content: receive_data.content.as_str().into(),
            attachments: smallvec![],
            author: Author {
                name: receive_data.author.name.as_str().into(),
                nickname: receive_data
                    .author_nick(self.serenity_ctx.read().await.http.clone())
                    .await,
                id: receive_data.author.id.0.to_string(),
                place: Place::Discord {
                    channel_id: receive_data.channel_id.0,
                },
            },
            timestamp: receive_data.timestamp,
        };

        let send_targets = self
            .channel_to_transceiver_bindings
            .read()
            .await
            .iter()
            .filter(|(_, ch_id)| *ch_id == receive_data.channel_id.0)
            .map(|(uuid, _)| *uuid)
            .collect::<Vec<_>>();

        let send_target_id = match send_targets.len() {
            0 => return Err(anyhow::Error::msg("no bound transceivers.")),
            1 => *send_targets.first().unwrap(),
            _ => unreachable!(), // FIXME: セキュリティ(DOS的処理の増大への懸念)的に見ても複数登録は駄目だと思うが, 一応審議
        };

        let res = match self.transceivers.get_child(send_target_id).await {
            Ok(child) => child.broadcast_send(data).await,
            Err(_) => unreachable!(),
        };

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub async fn send(&self, tr_id: Uuid, data: FormattedData) -> anyhow::Result<()> {
        let mut searched = self
            .channel_to_transceiver_bindings
            .read()
            .await
            .iter()
            .filter(|(id, _)| *id == tr_id)
            .map(|(_, id)| *id)
            .collect::<Vec<_>>();

        let ch_id = match searched.len() {
            0 => {
                return Err(anyhow::Error::msg(
                    "not found (transceiver id was not bound to discord channel_id).",
                ));
            }
            1 => searched.pop().unwrap(),
            _ => unreachable!(),
        };

        let res = ChannelId(ch_id)
            .say(self.serenity_ctx.read().await.http.clone(), data)
            .await;

        match res {
            Ok(msg) => {
                // TODO: log出力にでも吐こう - msg
                Ok(())
            }
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }
}

#[serenity::async_trait]
impl EventHandler for DiscordInterfaceWrapper {
    async fn message(&self, _: Context, msg: Message) {
        self.0.receive(msg).await.unwrap()
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        unsafe {
            std::ptr::write(self.0.serenity_ctx.write().await.deref_mut(), ctx);
        }
    }
}
