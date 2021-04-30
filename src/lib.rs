use crate::error::FTSyncError;
use crate::types::{Config, FTResult};
use std::collections::HashMap;

pub mod api;
pub mod commands;
pub mod config;
pub mod error;
pub mod types;

fn keys(_header: &ftd::p1::Header) -> HashMap<String, String> {
    todo!()
}

pub fn parse_config(filename: &str) -> FTResult<Config> {
    use std::fs;

    let contents = fs::read_to_string(filename).unwrap();
    let sections = ftd::p1::parse(contents.as_str())?;

    struct FTSync {
        repo: String,
        collection: String,
        backend: String,
        root: String,
    };

    let mut ftsync: Option<FTSync> = None;
    let mut ignored: Option<Vec<String>> = None;

    for section in sections {
        match section.name.as_str() {
            "ft-sync" => {
                let config_map = keys(&section.header);
                let repo = config_map.get("repo").ok_or_else(|| {
                    error::FTSyncError::ConfigFileParseError {
                        file: filename.to_string(),
                        error: "repo value not found".to_string(),
                    }
                })?;
                let collection = config_map.get("collection").ok_or_else(|| {
                    error::FTSyncError::ConfigFileParseError {
                        file: filename.to_string(),
                        error: "collection value not found".to_string(),
                    }
                })?;
                let root = config_map.get("root").ok_or_else(|| {
                    error::FTSyncError::ConfigFileParseError {
                        file: filename.to_string(),
                        error: "root value not found".to_string(),
                    }
                })?;
                let backend = config_map.get("backend").ok_or_else(|| {
                    error::FTSyncError::ConfigFileParseError {
                        file: filename.to_string(),
                        error: "backend value not found".to_string(),
                    }
                })?;

                ftsync = Some(FTSync {
                    repo: repo.to_string(),
                    collection: collection.to_string(),
                    backend: backend.to_string(),
                    root: root.to_string(),
                });
            }
            "ignored" => {
                ignored = Some(if let Some(body) = section.body.as_ref() {
                    body.lines()
                        .into_iter()
                        .filter_map(|line| {
                            if !line.trim().is_empty() {
                                Some(line.to_string())
                            } else {
                                None
                            }
                        })
                        .collect()
                } else {
                    vec![]
                });
            }
            t => {
                return Err(FTSyncError::ConfigFileParseError {
                    file: filename.to_string(),
                    error: format!("unknown section: {}", t),
                })?;
            }
        };
    }

    let ftsync = ftsync.ok_or_else(|| error::FTSyncError::ConfigFileParseError {
        file: filename.to_string(),
        error: "ftsync section not found".to_string(),
    })?;

    Config::new(
        ftsync.repo,
        ftsync.collection,
        ftsync.backend.as_str().into(),
        ftsync.root,
        ignored,
    )
}
