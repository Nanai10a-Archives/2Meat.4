use crate::model::Error;
use serenity::async_trait;
use serenity::client::ClientBuilder;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler};

pub async fn init(token: impl AsRef<str>) -> Result<(), Error> {
    let mut client = ClientBuilder::new(token)
        .guild_subscriptions(true)
        .event_handler(EventPoster)
        .await
        .unwrap();

    match client.start().await {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::Serenity(err)),
    }
}

struct EventPoster;

#[async_trait]
impl EventHandler for EventPoster {
    async fn message(&self, ctx: Context, message: Message) {
        if message.content == "!ping" {
            message.channel_id.say(ctx.http, "pong!").await.unwrap();
        }
    }
}