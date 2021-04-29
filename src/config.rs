use crate::types::*;
use std::path::Path;

impl <'a> Default for Config<'a> {
    fn default() -> Self {
        Config {
            ignored: vec![],
            repo: "".to_string(),
            collection: "".to_string(),
            backend: Backend::FTD,
            root: Path::new(""),
            mode: SyncMode::LocalToRemote,
            auth: Auth::Anonymous,
            dot_ft: false,
        }
    }
}

impl <'a>Config<'a> {
    pub fn set_repo(mut self, repo: &str) -> Self {
        self.repo = repo.to_string();
        self
    }

    pub fn set_collection(mut self, collection: &str) -> Self {
        self.collection = collection.to_string();
        self
    }

    pub fn set_root(mut self, root: &'a str) -> Self {
        self.root = Path::new(root);
        self
    }

    pub fn set_backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    pub fn add_ignored(mut self, ignored: gitignore::Pattern<'a>) -> Self {
        self.ignored.push(ignored);
        self
    }

    pub fn set_mode(mut self, mode: SyncMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn set_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }
}
