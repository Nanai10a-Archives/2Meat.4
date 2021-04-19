use crate::model::arg::Place;
use crate::utils::RefWrap;

#[serenity::async_trait]
pub trait NewCommand<T>: Sized {
    async fn new_com(parent: &T. place: Place) -> anyhow::Result<RefWrap<Self>>;
}

#[deprecated]
#[serenity::async_trait]
pub trait MutCommand {
    async fn mut_com(&self) -> anyhow::Result<()>;
}

#[serenity::async_trait]
pub trait DropCommand {
    async fn drop_com(&self) -> anyhow::Result<()>;
}

#[serenity::async_trait]
pub trait SubscCommand<T> {
    async fn subsc_com(&self, target: &T) -> anyhow::Result<()>;
}

#[serenity::async_trait]
pub trait ExitCommand<T> {
    async fn exit_com(&self, target: &T) -> anyhow::Result<()>;
}
