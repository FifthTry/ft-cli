mod action;
mod page;

pub use action::{action, ActionError, Result};
pub use page::{page, PageError, PageResult};

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}

#[derive(serde_derive::Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub result: Option<T>,
    // TODO: change to `pub error: std::collections::HashMap<String, String>,`
    pub error: Option<std::collections::HashMap<String, String>>,
}
