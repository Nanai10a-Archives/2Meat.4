use std::sync::Arc;

use uuid::Uuid;

use tokio::sync::{broadcast, mpsc, Mutex};

use crate::discord::transferer::Transferer;
use crate::model::data::FormattedData;
use crate::utils::RefWrap;
use futures_util::StreamExt;

pub type MxMpscTransceiver<T> = Mutex<(mpsc::Sender<T>, mpsc::Receiver<T>)>;

#[serenity::async_trait]
pub trait Transceivers {
    type Child: Transceiver;

    fn get_children(&self) -> Vec<RefWrap<Self::Child>>;

    async fn new(transferer: RefWrap<Transferer>) -> Self;
    async fn get_child(&self, id: Uuid) -> anyhow::Result<RefWrap<Self::Child>>;
    async fn new_child(&mut self) -> anyhow::Result<RefWrap<Self::Child>>;
    async fn on_child_drop(&mut self, id: Uuid) -> anyhow::Result<()>;

    async fn new_id(&self) -> Uuid;
}

#[serenity::async_trait]
pub trait Transceiver {
    type Parent: Transceivers;

    fn get_id(&self) -> Uuid;
    fn contain_subscribe_id(&self, id: Uuid) -> anyhow::Result<bool>;

    fn get_subscriber(&self) -> broadcast::Receiver<FormattedData>;

    fn subscribe_push(
        &mut self,
        id: Uuid,
        recv: broadcast::Receiver<FormattedData>,
    ) -> anyhow::Result<()>;
    fn subscribe_remove(&mut self, id: Uuid) -> anyhow::Result<()>;

    async fn drop_process(&self) -> anyhow::Result<()>;
}

pub struct DiscordTransceivers {
    children: Vec<(RefWrap<DiscordTransceiver>, MxMpscTransceiver<Signal>)>,
    transferer: RefWrap<Transferer>,
}

#[derive(Copy, Clone, Debug)]
pub enum Signal {
    Drop(Uuid),
    DropSuccess(Uuid),
}

pub struct DiscordTransceiver {
    id: Uuid,
    to_parent: MxMpscTransceiver<Signal>,
    subscribing: Vec<(Uuid, broadcast::Receiver<FormattedData>)>,
    broadcaster: broadcast::Sender<FormattedData>,
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

    async fn new(transferer: RefWrap<Transferer>) -> Self {
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
        let (m_send1, m_recv1) = mpsc::channel(8);
        let (m_send2, m_recv2) = mpsc::channel(8);

        let (b_send, _) = broadcast::channel(64);

        let child: RefWrap<Self::Child> = Arc::new(Mutex::new(Some(Self::Child {
            id,
            to_parent: Mutex::new((m_send1, m_recv2)),
            subscribing: vec![],
            broadcaster: b_send,
        })));

        let to_child = Mutex::new((m_send2, m_recv1));

        self.children.push((child.clone(), to_child));

        Ok(child)
    }

    async fn on_child_drop(&mut self, id: Uuid) -> anyhow::Result<()> {
        let stream = async_stream::stream! {
            for child_ref in self.get_children() {
                let res = match child_ref.lock().await.as_ref() {
                    None => None,
                    Some(child) => Some(child.get_id() == id),
                };

                yield match res {
                    None => None,
                    Some(is_match) => if is_match { Some(child_ref) } else { None }
                };
            }
        };

        futures_util::pin_mut!(stream);

        while let Some(value) = stream.next().await {
            match value {
                None => (),
                Some(_child_ref) => {
                    // 型の明示 (補完利いてほしいだけ).
                    let child_ref: RefWrap<DiscordTransceiver> = _child_ref;

                    let res = child_ref
                        .lock()
                        .await
                        .as_ref()
                        .unwrap()
                        .drop_process()
                        .await;

                    return match res {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    };
                }
            }
        }

        unreachable!()
    }

    #[allow(clippy::never_loop)]
    async fn new_id(&self) -> Uuid {
        loop {
            let id = self
                .transferer
                .lock()
                .await
                .as_mut()
                .unwrap()
                .new_id()
                .await;

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

    fn contain_subscribe_id(&self, id: Uuid) -> anyhow::Result<bool> {
        let filtered = self.subscribing.iter().filter(|item| (**item).0 == id);

        match filtered.count() {
            0 => Ok(false),
            1 => Ok(true),
            _ => todo!(),
        }
    }

    fn get_subscriber(&self) -> broadcast::Receiver<FormattedData> {
        self.broadcaster.subscribe()
    }

    fn subscribe_push(
        &mut self,
        id: Uuid,
        recv: broadcast::Receiver<FormattedData>,
    ) -> anyhow::Result<()> {
        if self.contain_subscribe_id(id).unwrap() {
            return Err(anyhow::Error::msg("already exists (duplicate id)."));
        }

        self.subscribing.push((id, recv));

        Ok(())
    }

    fn subscribe_remove(&mut self, id: Uuid) -> anyhow::Result<()> {
        if !self.contain_subscribe_id(id).unwrap() {
            return Err(anyhow::Error::msg("not found."));
        }

        let find_res = self
            .subscribing
            .iter()
            .map(|(got_id, _)| *got_id == id)
            .collect::<Vec<_>>();

        let mut index: Option<usize> = None;
        for i in 0..(find_res.len() - 1) {
            let res = *find_res.get(i).unwrap();

            if res {
                index = Some(i);
            }
        }

        if index.is_none() {
            todo!()
        }

        self.subscribing.remove(index.unwrap());

        Ok(())
    }

    // FIXME: どう考えてもこれdeadlockするよね
    async fn drop_process(&self) -> anyhow::Result<()> {
        let mut locked = self.to_parent.lock().await;

        let signal_id = Uuid::new_v4();

        let send_res = (*locked).0.send(Signal::Drop(signal_id)).await;
        match send_res {
            Ok(_) => (),
            Err(err) => return Err(anyhow::Error::new(err)),
        };

        loop {
            let recv_res = (*locked).1.recv().await;

            let recv_signal = match recv_res {
                None => continue,
                Some(s) => s,
            };

            match recv_signal {
                Signal::DropSuccess(recv_id) => {
                    if recv_id != signal_id {
                        unreachable!()
                    }

                    break Ok(());
                }
                _ => unreachable!(),
            }
        }
    }
}
