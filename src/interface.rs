use crate::model::data::FormattedData;

#[serenity::async_trait]
pub trait Interface {
    async fn receive(&self, data: FormattedData) -> anyhow::Result<()>;
    async fn send(&self, data: FormattedData) -> anyhow::Result<()>;
}
