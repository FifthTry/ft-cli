mod config;
pub mod error;
mod ftd;
pub mod git;
mod raw;
pub mod status;
pub mod sync;
pub mod types;
mod utils;

pub use crate::config::Config;
pub use error::Error;
pub use status::status;
pub use sync::sync;
pub use types::{Auth, Backend, FileMode, Result, SyncMode, User};

#[cfg(test)]
mod tests {

    #[test]
    fn fbt() {
        if fbt_lib::main().is_some() {
            panic!("test failed")
        }
    }
}

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}
