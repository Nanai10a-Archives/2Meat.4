use crate::model::data::FormattedData;

pub trait Interface {
    fn receive(&self, data: FormattedData) -> anyhow::Result<()>;
    fn send(&self, data: FormattedData) -> anyhow::Result<()>;
}
