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

    pub fn get(&self, id: Uuid) -> anyhow::Result<RefWrap<DiscordSender>> {
        let vec = self
            .senders
            .iter()
            .filter(|item| (***item).lock().unwrap().as_ref().unwrap().id == id)
            .collect::<Vec<_>>();
        match vec.len() {
            0..1 => (),
            _ => todo!(),
        };

        let arc = match vec.first() {
            None => return Err(anyhow::Error::msg("not found (was not registered).")),
            Some(arc) => (**arc).clone(),
        };

        let res = match *(*arc).lock().unwrap() {
            None => Err(anyhow::Error::msg("not found (was deleted).")),
            Some(_) => Ok(arc.clone()),
        };
        res
    }
}
