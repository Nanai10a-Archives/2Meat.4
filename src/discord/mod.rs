use serenity::client::ClientBuilder;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler};

mod impl_commands;
pub mod receivers;
pub mod senders;
pub mod transferer;

pub async fn init(token: impl AsRef<str>) -> anyhow::Result<()> {
    let mut client = ClientBuilder::new(token)
        // .event_handler(DiscordInterface {})
        .await
        .unwrap();

    tokio::spawn(async move { client.start().await.unwrap() });
    Ok(())
}
