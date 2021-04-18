use std::sync::Arc;

use uuid::Uuid;

use crate::discord::transferer::Transferer;
use crate::utils::RefWrap;

pub struct DiscordSender {
    id: Uuid,
    parent: Arc<DiscordSenders>,
}

impl DiscordSender {
    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub struct DiscordSenders {
    senders: Vec<RefWrap<DiscordSender>>,
    transferer: Arc<Transferer>,
}

impl DiscordSenders {
    pub fn new(transferer: Arc<Transferer>) -> Self {
        DiscordSenders {
            senders: vec![],
            transferer,
        }
    }

    pub async fn get(&self, id: Uuid) -> anyhow::Result<RefWrap<DiscordSender>> {
        let mut vec = vec![];
        for send in self.senders.iter() {
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
