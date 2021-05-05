#[macro_use]
extern crate serde_derive;

pub mod types;
pub mod api;
pub mod error;

pub type FTResult<T> = anyhow::Result<T>;