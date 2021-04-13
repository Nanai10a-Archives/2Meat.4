use std::sync::Arc;

use uuid::Uuid;

use crate::discord::transferer::Transferer;

pub struct DiscordReceiver {
    id: Uuid,
    parent: Arc<DiscordReceivers>,
}

impl DiscordReceiver {
    pub fn id(&self) -> Uuid {
        self.id
    }
}

pub struct DiscordReceivers {
    receivers: Vec<Arc<Option<DiscordReceiver>>>,
    transferer: Arc<Transferer>,
}

impl DiscordReceivers {
    pub fn new(transferer: Arc<Transferer>) -> Self {
        DiscordReceivers {
            receivers: vec![],
            transferer,
        }
    }

    pub fn get(&self, id: Uuid) -> anyhow::Result<Arc<Option<DiscordReceiver>>> {
        let vec = self
            .receivers
            .iter()
            .filter(|item| match ***item {
                None => false,
                Some(_) => (***item).unwrap().id == id,
            })
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
