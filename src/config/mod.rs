pub mod section;

use crate::types::*;
use crate::error::FTSyncError;

pub struct Config {
    pub ignored: Vec<String>,
    pub repo: String,
    pub collection: String,
    pub backend: Backend,
    pub root: String,
    pub mode: SyncMode,
    pub auth: Auth,
    pub dot_ft: bool,
}

impl Config {
    pub fn from_file(filename: &str) -> FTResult<Self> {
        use std::fs;
        let contents = fs::read_to_string(name)?;
        Self::parse(contents.as_str())
    }

    pub fn parse(content: &str) -> FTResult<Self> {
        let p1 = ftd::p1::parse(content)?;
        let mut sections = vec![];

        let mut ftsync: Option<section::FtSync> = None;
        let mut ignored: Vec<section::Ignored> = vec![];
        for section in p1 {
            let s = crate::config::Section::from_p1(&section)?;
            match s {
                section::FtSync(sec) => {
                    if ftsync.is_none() {
                        ftsync = Some(sec)
                    }
                    else {
                        return Err(FTSyncError::ConfigFileParseError {error: "Duplicate FTSync section".to_string()});
                    }
                },
                section::Ignored(sec) => ignored.push(sec)
            }
        };

        let ftsync = match ftsync {
            Some(f) => f,
            None =>
                return Err(FTSyncError::ConfigFileParseError {error: "No FTSync section found".to_string()});
        };

        let patterns = ignored.iter().flat_map(|ig| ig.patterns).collect();

        Ok(Config {
            ignored: patterns,
            repo: ftsync.repo,
            collection: ftsync.collection,
            backend: ftsync.backend.into(),
            root: ftsync.root,
            mode: SyncMode::LocalToRemote,
            auth: Auth::Anonymous,
            dot_ft: false
        })
    }
}
