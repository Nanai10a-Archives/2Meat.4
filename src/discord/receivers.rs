use std::sync::Arc;

use uuid::Uuid;

use crate::discord::transferer::Transferer;
use crate::utils::RefWrap;

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
    receivers: Vec<RefWrap<DiscordReceiver>>,
    transferer: Arc<Transferer>,
}

impl DiscordReceivers {
    pub fn new(transferer: Arc<Transferer>) -> Self {
        DiscordReceivers {
            receivers: vec![],
            transferer,
        }
    }

    pub fn get(&self, id: Uuid) -> anyhow::Result<RefWrap<DiscordReceiver>> {
        let vec = self
            .receivers
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

        match *(*arc).lock().unwrap() {
            None => Err(anyhow::Error::msg("not found (was deleted).")),
            Some(_) => Ok(arc),
        }
    }
}
