pub mod commands;
pub mod types;

fn parse_config(name: &str) -> crate::types::Config {
    use ftd;
    use std::fs;

    let contents = fs::read_to_string(filename).unwrap();
    let sections = ftd::Document::parse(contents.as_str(), name)?;

    for section in sections {}

    crate::types::Config {}
}
