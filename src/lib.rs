use crate::types::Config;
use std::collections::HashMap;

pub mod api;
pub mod commands;
pub mod config;
pub mod types;
pub mod ftd_parse;

fn keys(header: ftd::p1::Header) -> HashMap<String, String> {
    todo!()
}

pub fn parse_config(name: &str) -> crate::types::Config {
    use ftd;
    use std::fs;

    let contents = fs::read_to_string(name).unwrap();
    let sections = ftd::p1::parse(contents.as_str()).unwrap();

    let mut config = Config::default();
    let mut ignored_lines: Vec<String> = vec![];

    for section in sections {
        match section.name.as_str() {
            "ft-sync" => {
                let config_map = keys(section.header);
                let repo = config_map.get("repo").unwrap();
                let collection = config_map.get("collection").unwrap();
                let root = config_map.get("root").unwrap();
                let backend = config_map.get("backend").unwrap();

                // config = config
                //     .set_repo(repo)
                //     .set_backend(backend.as_str().into())
                //     .set_root(root)
                //     .set_collection(collection);
            }
            "ignored" => {
                if let Some(body) = section.body {
                    for line in body.lines() {
                        if !line.trim().is_empty() {
                            ignored_lines.push(line.to_string());
                        }
                    }
                }
            }
            _ => {
                todo!()
            }
        }
    }

    for line in ignored_lines {
        let pattern = gitignore::Pattern::new(line.as_str(), &config.root).unwrap();
        config = config.add_ignored(pattern);
    }
    config
}
