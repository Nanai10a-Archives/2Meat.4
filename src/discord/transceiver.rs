use std::sync::Arc;

use uuid::Uuid;

use crate::discord::transferer::Transferer;
use crate::utils::RefWrap;

#[serenity::async_trait]
pub trait Transceivers {
    type Child: Transceiver;

    async fn new(transferer: Arc<Transferer>) -> Self;
    async fn get_child(&self, id: Uuid) -> anyhow::Result<RefWrap<Self::Child>>;
}

#[serenity::async_trait]
pub trait Transceiver {
    type Parent: Transceivers;
}

pub struct DiscordTransceivers {
    children: Vec<RefWrap<DiscordTransceiver>>,
    transferer: Arc<Transferer>,
}

pub struct DiscordTransceiver {
    id: Uuid,
    parent: Arc<DiscordTransceivers>,
}

#[serenity::async_trait]
impl Transceivers for DiscordTransceivers {
    type Child = DiscordTransceiver;

    async fn new(transferer: Arc<Transferer>) -> Self {
        DiscordTransceivers {
            children: vec![],
            transferer,
        }
    }

    async fn get_child(&self, id: Uuid) -> anyhow::Result<RefWrap<Self::Child>> {
        let mut vec = vec![];
        for send in self.children.iter() {
            if (**send).lock().await.as_ref().unwrap().id == id {
                vec.push(send);
            }
        }

        match vec.len() {
            0..1 => (),
            _ => todo!(),
        };

        let arc = match vec.first() {
            None => return Err(anyhow::Error::msg("not found (was not registered).")),
            Some(arc) => (*arc).clone(),
        };

        let res = match *(*arc).lock().await {
            None => Err(anyhow::Error::msg("not found (was deleted).")),
            Some(_) => Ok(arc.clone()),
        };
        res
    }
}

#[serenity::async_trait]
impl Transceiver for DiscordTransceiver {
    type Parent = DiscordTransceivers;
}
