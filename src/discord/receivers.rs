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

    pub async fn get(&self, id: Uuid) -> anyhow::Result<RefWrap<DiscordReceiver>> {
        let mut vec = vec![];
        for send in self.receivers.iter() {
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
