use crate::commands;
use crate::discord::receivers::{DiscordReceiver, DiscordReceivers};
use crate::discord::senders::{DiscordSender, DiscordSenders};
use crate::model::arg::Place;
use crate::utils::RefWrap;

impl commands::New<DiscordSenders> for DiscordSender {
    fn new(_parent: &DiscordSenders, _place: Place) -> anyhow::Result<RefWrap<Self>> {
        todo!()
    }
}

impl commands::Drop for DiscordSender {
    fn drop(_self_: RefWrap<Self>) -> anyhow::Result<()> {
        todo!()
    }
}

impl commands::New<DiscordReceivers> for DiscordReceiver {
    fn new(_parent: &DiscordReceivers, _place: Place) -> anyhow::Result<RefWrap<Self>> {
        todo!()
    }
}

impl commands::Drop for DiscordReceiver {
    fn drop(_self_: RefWrap<Self>) -> anyhow::Result<()> {
        todo!()
    }
}

impl commands::Subsc<DiscordSender> for DiscordReceiver {
    fn subsc(&mut self, _target: &DiscordSender) -> anyhow::Result<()> {
        todo!()
    }
}

impl commands::Exit<DiscordSender> for DiscordReceiver {
    fn exit(&mut self, _target: &DiscordSender) -> anyhow::Result<()> {
        todo!()
    }
}
