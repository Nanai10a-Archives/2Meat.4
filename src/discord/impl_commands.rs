use crate::commands;
use crate::discord::receivers::{DiscordReceiver, DiscordReceivers};
use crate::discord::senders::{DiscordSender, DiscordSenders};
use crate::model::arg::Place;

impl commands::New<DiscordSenders> for DiscordSender {
    fn new(_parent: impl AsRef<DiscordSenders>, _place: Place) -> anyhow::Result<Box<Self>> {
        todo!()
    }
}

impl commands::Drop for DiscordSender {
    fn drop(self) -> anyhow::Result<()> {
        todo!()
    }
}

impl commands::New<DiscordReceivers> for DiscordReceiver {
    fn new(_parent: impl AsRef<DiscordReceivers>, _place: Place) -> anyhow::Result<Box<Self>> {
        todo!()
    }
}

impl commands::Drop for DiscordReceiver {
    fn drop(self) -> anyhow::Result<()> {
        todo!()
    }
}

impl commands::Subsc<DiscordSender> for DiscordReceiver {
    fn subsc(&self, _target: impl AsRef<DiscordSender>) -> anyhow::Result<()> {
        todo!()
    }
}

impl commands::Exit<DiscordSender> for DiscordReceiver {
    fn exit(&self, _target: impl AsRef<DiscordSender>) -> anyhow::Result<()> {
        todo!()
    }
}
