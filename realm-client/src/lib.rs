mod action;
mod client;
mod page;
mod types;

pub use action::action;
pub(crate) use client::{client, handle, url};
pub use page::page;
pub use types::{Error, Result};

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}
