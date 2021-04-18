use crate::model::arg::Place;
use crate::utils::RefWrap;

pub trait NewCommand<T> {
    fn new_com(parent: &T, place: Place) -> anyhow::Result<RefWrap<Self>>;
}

#[deprecated]
pub trait MutCommand {
    fn mut_com(&self) -> anyhow::Result<()>;
}

pub trait DropCommand {
    fn drop_com(&self) -> anyhow::Result<()>;
}

pub trait SubscCommand<T> {
    fn subsc_com(&self, target: &T) -> anyhow::Result<()>;
}

pub trait ExitCommand<T> {
    fn exit_com(&self, target: &T) -> anyhow::Result<()>;
}
