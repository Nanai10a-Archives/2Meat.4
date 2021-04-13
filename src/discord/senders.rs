use std::sync::Arc;

use uuid::Uuid;

use crate::discord::transferer::Transferer;

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
    senders: Vec<Arc<Option<DiscordSender>>>,
    transferer: Arc<Transferer>,
}

impl DiscordSenders {
    pub fn new(transferer: Arc<Transferer>) -> Self {
        DiscordSenders {
            senders: vec![],
            transferer,
        }
    }

    pub fn get(&self, id: Uuid) -> anyhow::Result<Arc<Option<DiscordSender>>> {
        let vec = self
            .senders
            .iter()
            .filter(|item| item.id == id)
            .collect::<Vec<_>>();
        match vec.len() {
            0..1 => (),
            _ => todo!(),
        };

        let arc = match vec.first() {
            None => return Err(anyhow::Error::msg("not found (was not registered).")),
            Some(arc) => (**arc).clone(),
        };

        match *arc {
            None => Err(anyhow::Error::msg("not found (was deleted).")),
            Some(_) => Ok(arc),
        }
    }
}
