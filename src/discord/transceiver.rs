use std::sync::Arc;

use uuid::Uuid;

use tokio::sync::{mpsc, Mutex};

use crate::discord::transferer::Transferer;
use crate::utils::RefWrap;
use futures_util::StreamExt;

pub type MxMpscTransceiver<T> = Mutex<(mpsc::Sender<T>, mpsc::Receiver<T>)>;

#[serenity::async_trait]
pub trait Transceivers {
    type Child: Transceiver;

    fn get_children(&self) -> Vec<RefWrap<Self::Child>>;

    async fn new(transferer: Arc<Transferer>) -> Self;
    async fn get_child(&self, id: Uuid) -> anyhow::Result<RefWrap<Self::Child>>;
    async fn new_child(&mut self) -> anyhow::Result<RefWrap<Self::Child>>;

    async fn new_id(&self) -> Uuid;
}

#[serenity::async_trait]
pub trait Transceiver {
    type Parent: Transceivers;

    fn get_id(&self) -> Uuid;
}

pub struct DiscordTransceivers {
    children: Vec<(RefWrap<DiscordTransceiver>, MxMpscTransceiver<Signal>)>,
    transferer: Arc<Transferer>,
}

pub enum Signal {
    Drop(Uuid),
    DropSuccess(Uuid),
}

pub struct DiscordTransceiver {
    id: Uuid,
    to_parent: MxMpscTransceiver<Signal>,
}

#[serenity::async_trait]
impl Transceivers for DiscordTransceivers {
    type Child = DiscordTransceiver;

    fn get_children(&self) -> Vec<RefWrap<Self::Child>> {
        self.children
            .iter()
            .map(|item| item.0.clone())
            .collect::<Vec<_>>()
    }

    async fn new(transferer: Arc<Transferer>) -> Self {
        DiscordTransceivers {
            children: vec![],
            transferer,
        }
    }

    async fn get_child(&self, id: Uuid) -> anyhow::Result<RefWrap<Self::Child>> {
        let mut vec = vec![];
        for send in self.children.iter() {
            if send.0.lock().await.as_ref().unwrap().id == id {
                vec.push(send.0.clone());
            }
        }

        match vec.len() {
            0..1 => (),
            _ => todo!(),
        };

        let arc = match vec.first() {
            None => return Err(anyhow::Error::msg("not found (was not registered).")),
            Some(arc) => arc.clone(),
        };

        let res = match *arc.lock().await {
            None => Err(anyhow::Error::msg("not found (was deleted).")),
            Some(_) => Ok(arc.clone()),
        };
        res
    }

    async fn new_child(&mut self) -> anyhow::Result<RefWrap<Self::Child>> {
        let id = self.new_id().await;
        let (send1, recv1) = mpsc::channel(8);
        let (send2, recv2) = mpsc::channel(8);

        let child: RefWrap<Self::Child> = Arc::new(Mutex::new(Some(Self::Child {
            id,
            to_parent: Mutex::new((send1, recv2)),
        })));

        let to_child = Mutex::new((send2, recv1));

        self.children.push((child.clone(), to_child));

        Ok(child)
    }

    #[allow(clippy::never_loop)]
    async fn new_id(&self) -> Uuid {
        loop {
            let id = Uuid::new_v4();

            let stream = async_stream::stream! {
                for child_ref in self.get_children() {
                    yield match child_ref.lock().await.as_ref() {
                        None => false,
                        Some(child) => child.get_id() == id,
                    };
                }
            };

            futures_util::pin_mut!(stream);

            while let Some(value) = stream.next().await {
                if value {
                    continue;
                }
            }

            break id;
        }
    }
}

#[serenity::async_trait]
impl Transceiver for DiscordTransceiver {
    type Parent = DiscordTransceivers;

    fn get_id(&self) -> Uuid {
        self.id
    }
}
