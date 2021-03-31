extern crate two_meat_rust;

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::ClientBuilder;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};
use serenity::{async_trait, Error};
use threadpool::ThreadPool;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut client =
        ClientBuilder::new(std::env::var("DISCORD_TOKEN").expect("Error: token not found!"))
            .guild_subscriptions(true)
            .event_handler(EventPoster)
            .await
        {
            Ok(client) => client,
            Err(err) => panic!("{}", err),
        };

    match client.start().await {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    }
}

struct EventPoster;

#[async_trait]
impl EventHandler for EventPoster {
    async fn message(&self, ctx: Context, message: Message) {
        if message.content == "!ping" {
            message.channel_id.say(ctx.http, "pong!").await;
        }
    }
}
