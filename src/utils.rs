use std::sync::{Arc, Mutex};

pub type RefWrap<T> = Arc<Mutex<Option<T>>>;
