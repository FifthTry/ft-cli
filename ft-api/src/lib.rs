#[macro_use]
extern crate serde_derive;

pub mod api;
pub mod bulk_update;
pub mod error;
pub mod sync_status;

pub type Result<T> = anyhow::Result<T>;

pub use bulk_update::bulk_update;
pub use sync_status::sync_status;
