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

    let out = format!("{}.out.json", tid.as_str());
    std::fs::write(
        out.as_str(),
        sorted_json::to_json(&serde_json::to_value(input).unwrap()),
    )
    .unwrap_or_else(|e| panic!("failed to write to: {}, err={:?}", out, e));

    let input = format!("{}.in.json", tid.as_str());

    serde_json::from_str(
        std::fs::read_to_string(input.as_str())
            .unwrap_or_else(|e| panic!("failed to read from: {}, err={:?}", input, e))
            .as_str(),
    )
    .expect("failed to parse json")
}
