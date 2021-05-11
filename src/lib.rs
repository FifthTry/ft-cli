pub mod config;
pub mod error;
pub mod git;
pub mod status;
pub mod sync;
pub mod types;

pub use crate::config::Config;
pub use error::Error;
pub use status::status;
pub use sync::sync;
pub use types::{Auth, Backend, Result, SyncMode, User};

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
