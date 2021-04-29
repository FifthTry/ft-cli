use crate::types::Config;
use std::collections::HashMap;

pub mod commands;
pub mod config;
pub mod types;

fn keys(header: ftd::p1::Header) -> HashMap<String, String> {
    todo!()
}

fn parse_config(name: &str) -> crate::types::Config {
    use ftd;
    use std::fs;

    let contents = fs::read_to_string(filename).unwrap();
    let sections = ftd::p1::parse(contents.as_str())?;

    let mut config = Config::default();
    let mut ignored_lines: Vec<String> = vec![];

    for section in sections {
        match section.name.as_str() {
            "ft-sync" => {
                let config_map = keys(section.header);
                let repo = config_map.get("repo")?;
                let collection = config_map.get("collection")?;
                let root = config_map.get("root")?;
                let backend = config_map.get("backend")?;

                config
                    .set_repo(repo)
                    .set_backend(backend.into())
                    .set_root(root)
                    .set_collection(collection);
            }
            "ignored" => {
                if let Some(body) = section.body {
                    for line in body.lines() {
                        if !line.trim().is_empty() {
                            ignored_lines.push(line.to_string());
                        } else {
                            None
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
        let pattern = gitignore::Pattern::new(line.as_str(), config.root)?;
        config.add_ignored(pattern);
    }
    config
}
