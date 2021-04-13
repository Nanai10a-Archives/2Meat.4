use crate::model::arg::Place;
use std::sync::Arc;

pub trait New<T> {
    fn new(parent: impl AsRef<T>, place: Place) -> anyhow::Result<Box<Self>>;
}

#[deprecated]
pub trait Mut {
    fn mut_(&self) -> anyhow::Result<()>;
}

pub trait Drop {
    fn drop(self) -> anyhow::Result<()>;
}

pub trait Subsc<T> {
    fn subsc(&self, target: impl AsRef<T>) -> anyhow::Result<()>;
}

pub trait Exit<T> {
    fn exit(&self, target: impl AsRef<T>) -> anyhow::Result<()>;
}
