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
