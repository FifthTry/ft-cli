mod action;
mod client;
mod env;
mod page;
mod types;

pub use action::action;
pub(crate) use client::{client, handle, url};
pub use page::page;
pub use types::{Error, Result};

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}

pub fn mock<T1, T2>(tid: Option<String>, input: T1) -> T2
where
    T1: serde::Serialize,
    T2: serde::de::DeserializeOwned,
{
    let tid = match tid {
        Some(v) => v,
        None => panic!("tid is none in test mode"),
    };

    // write to ./tid.url and return content of tid.json
    std::fs::write(
        format!("{}.out.json", tid.as_str()),
        sorted_json::to_json(&serde_json::to_value(input).unwrap()),
    )
    .expect("failed to write to .out.json file");

    serde_json::from_str(
        std::fs::read_to_string(format!("{}.in.json", tid.as_str()))
            .expect("failed to read .json file")
            .as_str(),
    )
    .expect("failed to parse json")
}
