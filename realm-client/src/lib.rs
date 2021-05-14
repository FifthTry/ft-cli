mod action;
mod page;
mod types;

pub use action::action;
pub use page::page;
pub use types::{Error, Result};

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}
