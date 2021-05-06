#[macro_use]
extern crate serde_derive;

pub mod types;
pub mod api;
pub mod error;
pub mod bulk_update;
pub mod status;
pub type FTResult<T> = anyhow::Result<T>;
pub use types::*;