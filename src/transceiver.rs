use std::sync::{Arc, Weak};

use futures_util::StreamExt;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Transceiverのinstance
pub struct Transceiver {
    /// 一意なUUID Transceiversから与えられる
    pub id: Uuid,
    /// self(instance)がbroadcastする相手のList
    subscribers: RwLock<Vec<Uuid>>,
    /// Transceiversへの参照
    parent: Arc<Transceivers>,
}

/// Transceiverの統括をする
pub struct Transceivers {
    /// instanceのList
    children: RwLock<Vec<Arc<Transceiver>>>,
    /// 自身への弱参照
    weak: Weak<Self>,
}

impl Transceiver {
    pub async fn add_subscribe(&self, broadcaster_id: Uuid) {
        self.subscribers.write().await.push(broadcaster_id)
    }

    pub async fn remove_subscribe(&self, broadcaster_id: Uuid) -> anyhow::Result<()> {
        let res = self
            .subscribers
            .read()
            .await
            .iter()
            .enumerate()
            .filter(|(_, item)| **item == broadcaster_id)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        match res.len() {
            0 => Err(anyhow::Error::msg(
                "not found (cannot find subscribing instance).",
            )),
            1 => {
                let index = res.first().unwrap();
                self.subscribers.write().await.remove(*index);

                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

impl Transceivers {
    fn new_in_arc() -> Arc<Self> {
        let mut arc = Arc::new_uninit();

        let deceiving_arc = unsafe { arc.clone().assume_init() };

        let item = Self {
            children: RwLock::new(Vec::new()),
            weak: Arc::downgrade(&deceiving_arc),
        };

        unsafe {
            Arc::get_mut_unchecked(&mut arc).as_mut_ptr().write(item);

            arc.assume_init()
        }
    }

    fn get_arc(&self) -> Arc<Self> {
        self.weak.clone().upgrade().unwrap()
    }

    #[allow(clippy::never_loop)]
    async fn new_id(&self) -> Uuid {
        loop {
            let new_id = Uuid::new_v4();

            let duplicate_count = self
                .children
                .read()
                .await
                .iter()
                .filter(|item| item.id == new_id)
                .count();

            match duplicate_count {
                0 => break new_id,
                1 => continue,
                _ => unreachable!(),
            }
        }
    }

    pub async fn new_instance(&self) -> Arc<Transceiver> {
        let child = Arc::new(Transceiver {
            id: self.new_id().await,
            subscribers: RwLock::new(Vec::new()),
            parent: self.get_arc(),
        });

        self.children.write().await.push(child.clone());

        child
    }
}
