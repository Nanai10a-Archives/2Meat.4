#![feature(async_closure)]
#![feature(exclusive_range_pattern)]
pub mod commands;
pub mod discord;
pub mod interface;
pub mod model;
pub mod prelude;
pub mod utils;

fn unchecked<E>() -> Box<dyn Fn(E) -> ()> {
    Box::new(|_| unimplemented!("implementer was not checked this handling... sorry."))
}
// .unwrap_or_else(crate::unchecked().as_ref())
