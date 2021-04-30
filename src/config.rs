use crate::error::FTSyncError;
use crate::types::*;

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config {
            ignored: vec![],
            repo: "".to_string(),
            collection: "".to_string(),
            backend: Backend::FTD,
            root: "".to_string(),
            mode: SyncMode::LocalToRemote,
            auth: Auth::Anonymous,
            dot_ft: false,
        }
    }
}

impl<'a> Config<'a> {
    pub fn new(
        repo: String,
        collection: String,
        backend: Backend,
        root: String,
        ignored: Option<Vec<String>>,
    ) -> FTResult<Self> {
        use gitignore::Pattern;

        let mut patterns: Vec<Pattern> = vec![];

        let mut config = Config {
            ignored: patterns,
            repo,
            collection,
            backend,
            root,
            mode: SyncMode::LocalToRemote,
            auth: Auth::Anonymous,
            dot_ft: false,
        };

        for p in ignored.unwrap_or(vec![]) {
            config.ignored.push(
                gitignore::Pattern::new(p.as_str(), std::path::Path::new(config.root.as_str()))
                    .map_err(|e| FTSyncError::ConfigFileParseError {
                        file: "".to_string(),
                        error: format!("error in ignored pattern {:?}", e),
                    })?,
            );
        }

        Ok(config)
    }
}
