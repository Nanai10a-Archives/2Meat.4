use crate::model::arg::Place;
use crate::utils::RefWrap;

pub trait New<T>: Sized {
    fn new(parent: &T, place: Place) -> anyhow::Result<RefWrap<Self>>;
}

#[deprecated]
pub trait Mut {
    fn mut_(&mut self) -> anyhow::Result<()>;
}

pub trait Drop: Sized {
    fn drop(self_: RefWrap<Self>) -> anyhow::Result<()>;
}

pub trait Subsc<T> {
    fn subsc(&mut self, target: &T) -> anyhow::Result<()>;
}

pub trait Exit<T> {
    fn exit(&mut self, target: &T) -> anyhow::Result<()>;
}
