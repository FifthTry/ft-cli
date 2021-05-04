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
        let contents = fs::read_to_string(filename)?;
        Self::parse(contents.as_str())
    }

    pub fn parse(content: &str) -> FTResult<Self> {
        let p1 = ftd::p1::parse(content)?;
        let mut ftsync: Option<section::FtSync> = None;
        let mut ignored: Vec<section::Ignored> = vec![];
        for section in p1 {
            let s = section::Section::from_p1(&section)?;
            match s {
                section::Section::FtSync(sec) => {
                    if ftsync.is_none() {
                        ftsync = Some(sec)
                    }
                    else {
                        return Err(FTSyncError::ConfigFileParseError {error: "Duplicate FTSync section".to_string()}.into());
                    }
                },
                section::Section::Ignored(sec) => ignored.push(sec)
            }
        };

        let ftsync = match ftsync {
            Some(f) => f,
            None =>
                return Err(
                    FTSyncError::ConfigFileParseError {error: "No FTSync section found".to_string()}.into()
                )
        };

        let patterns = ignored.into_iter().flat_map(|ig| ig.patterns).collect();

        Ok(Config {
            ignored: patterns,
            repo: ftsync.repo,
            collection: ftsync.collection,
            backend: ftsync.backend.as_str().into(),
            root: ftsync.root,
            mode: SyncMode::LocalToRemote,
            auth: Auth::AuthCode("ZV6cN8i6B8VUrb5PgPKc".to_string()),
            dot_ft: false
        })
    }
}
