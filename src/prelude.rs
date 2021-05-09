use std::time::Duration;

use crate::interface::discord::DiscordInterface;
use crate::transceiver::Transceivers;

pub struct TwoMeatSystem;

impl TwoMeatSystem {
    pub async fn boot(discord_token: impl AsRef<str>) -> anyhow::Result<()> {
        let transceivers = Transceivers::new_in_arc();

        let discord = DiscordInterface::new(discord_token, transceivers.clone()).await?;

        tokio::time::sleep(Duration::new(u64::MAX, 0)).await; // FIXME: 終了をどうやって阻止しよう？

        Ok(())
    }
}
