use crate::types::*;

impl Default for Config {
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

impl Config {
    fn set_repo(mut self, repo: &str) -> Self {
        self.repo = repo.to_string();
        self
    }

    fn set_collection(mut self, collection: &str) -> Self {
        self.collection = collection.to_string();
        self
    }

    fn set_root(mut self, root: &str) -> Self {
        self.root = root.to_string();
        self
    }

    fn set_backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    fn add_ignored(mut self, ignored: gitignore::Pattern) -> Self {
        self.ignored.push(gitignore);
        self
    }

    fn set_mode(mut self, mode: SyncMode) -> Self {
        self.mode = mode;
        self
    }

    fn set_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }
}
