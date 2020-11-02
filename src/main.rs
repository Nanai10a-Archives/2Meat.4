extern crate two_meat_rust;

use log::{error, info, LevelFilter};
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::ClientBuilder;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};
use serenity::{async_trait, Error};
use simplelog::Config;
use threadpool::ThreadPool;

#[tokio::main]
async fn main() {
    simplelog::SimpleLogger::init(LevelFilter::Info, Config::default()).expect("err!");

    info!("initialized env_logger");

    let mut client =
        ClientBuilder::new("")
            .guild_subscriptions(true)
            .event_handler(EventPoster)
            .await;
    info!("created client");

    let mut _client = match client {
        Ok(ok) => ok,
        Err(err) => {
            return error!("{}", err.to_string().as_str());
        }
    };

    match _client.start().await {
        Ok(_) => {}
        Err(err) => {
            return error!("{}", err.to_string().as_str());
        }
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
