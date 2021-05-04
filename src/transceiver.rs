use std::sync::{Arc, Weak};

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

    async fn find_child(&self, child_id: Uuid) -> Vec<Arc<Transceiver>> {
        self.children
            .read()
            .await
            .iter()
            .filter(|item| item.id == child_id)
            .map(|item| (*item).clone())
            .collect()
    }

    pub async fn get_child(&self, child_id: Uuid) -> anyhow::Result<Arc<Transceiver>> {
        let mut res = self.find_child(child_id).await;
        match res.len() {
            0 => Err(anyhow::Error::msg("not found.")),
            1 => Ok(res.pop().unwrap()),
            _ => unreachable!(),
        }
    }

    pub async fn child_exists(&self, child_id: Uuid) -> bool {
        match self.find_child(child_id).await.len() {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }

    async fn check_broadcaster_and_get_subscriber(
        &self,
        subscriber_id: Uuid,
        broadcaster_id: Uuid,
    ) -> anyhow::Result<Arc<Transceiver>> {
        let subscriber = match self.get_child(subscriber_id).await {
            Ok(ok) => ok,
            Err(_) => return Err(anyhow::Error::msg("not found (from subscriber_id).")),
        };

        if !self.child_exists(broadcaster_id).await {
            return Err(anyhow::Error::msg("not found (from broadcaster_id)."));
        }

        Ok(subscriber)
    }

    pub async fn add_subscribe_child(
        &self,
        subscriber_id: Uuid,
        broadcaster_id: Uuid,
    ) -> anyhow::Result<()> {
        let subscriber = self
            .check_broadcaster_and_get_subscriber(subscriber_id, broadcaster_id)
            .await?;

        subscriber.add_subscribe(broadcaster_id).await;

        Ok(())
    }

    pub async fn remove_subscribe_child(
        &self,
        subscriber_id: Uuid,
        broadcaster_id: Uuid,
    ) -> anyhow::Result<()> {
        let subscriber = self
            .check_broadcaster_and_get_subscriber(subscriber_id, broadcaster_id)
            .await?;

        subscriber.remove_subscribe(broadcaster_id).await?;

        Ok(())
    }

    pub async fn drop_child(&self, child_id: Uuid) -> anyhow::Result<()> {
        let child = self.get_child(child_id).await?;

        for item in self.children.write().await.iter() {
            let mut item_subscribers = item.subscribers.write().await;
            if item_subscribers.contains(&child_id) {
                let searched = item_subscribers
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| **item == child_id)
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();

                let index = match searched.len() {
                    1 => *searched.first().unwrap(),
                    _ => unreachable!(),
                };

                // FIXME: この処理は果たして必要か (勝手に削除してしまっていいのか / dropped flagを立てるだけでもいいのでは？)
                item_subscribers.remove(index);
            }
        }

        {
            let mut children = self.children.write().await;
            let searched = children
                .iter()
                .enumerate()
                .filter(|(_, item)| Arc::ptr_eq(item, &child))
                .map(|(index, _)| index)
                .collect::<Vec<_>>();

            let index = match searched.len() {
                1 => *searched.first().unwrap(),
                _ => unreachable!(),
            };
            children.remove(index);
        }

        let ref_count = Arc::strong_count(&child);

        match ref_count {
            0 => panic!("illegal reference count number detected!"),
            1 => Ok(()),
            _ => Err(anyhow::Error::msg(
                "could not drop (reference count not zero).",
            )),
        }
    }
}
