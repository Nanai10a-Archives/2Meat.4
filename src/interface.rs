pub trait Interface<RD, SD> {
    fn receive(&self, data: RD) -> anyhow::Result<()>;
    fn send(&self, data: SD) -> anyhow::Result<()>;
}
