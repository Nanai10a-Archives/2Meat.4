use std::sync::Arc;
use tokio::sync::Mutex;

pub type RefWrap<T> = Arc<Mutex<Option<T>>>;
