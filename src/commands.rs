pub trait New<T>: Sized {
    fn new(parent: &T, place: Place) -> anyhow::Result<Self>;
}

pub enum Place {
    There,
    // TODO: FEATURE - Meta
}

#[deprecated]
pub trait Mut: Sized {
    fn mut_(self) -> anyhow::Result<Self>;
}

pub trait Drop: Sized {
    fn drop(self) -> anyhow::Result<()>;
}

pub trait Subsc<T>: Sized {
    fn subsc(self, target: &T) -> anyhow::Result<Self>;
}

pub trait Exit<T>: Sized {
    fn exit(self, target: &T) -> anyhow::Result<Self>;
}
