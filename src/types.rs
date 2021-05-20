pub type Result<T> = std::result::Result<T, crate::Error>;

pub enum Auth {
    SignedIn(User),
    AuthCode(String),
    Anonymous,
}

pub struct User {
    pub cookie: String,
    pub username: String,
    pub name: String,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Backend {
    FTD,
}

impl Backend {
    pub fn from(s: &str) -> Option<Backend> {
        match s {
            "ftd" => Some(Backend::FTD),
            _ => None,
        }
    }
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Backend::FTD => write!(f, "ftd"),
        }
    }
}

pub enum SyncMode {
    LocalToRemote,
    RemoteToLocal,
    TwoWay,
}

pub enum FileMode {
    Deleted(String),
    Added(String),
    Modified(String),
}

impl FileMode {
    pub fn id(&self, root_dir: &str, collection: &str) -> String {
        let t = self
            .path()
            .strip_prefix(root_dir)
            .unwrap()
            .with_extension("")
            .to_str()
            .unwrap()
            .to_string();

        if t == "index" {
            collection.to_string()
        } else {
            collection.to_string() + "/" + t.as_str()
        }
    }

    pub fn content(&self) -> crate::Result<String> {
        std::fs::read_to_string(self.path())
            .map_err(|e| crate::Error::ReadError(e, self.path_str()))
    }

    pub fn path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(match self {
            FileMode::Added(v) => v,
            FileMode::Deleted(v) => v,
            FileMode::Modified(v) => v,
        })
    }

    pub fn path_str(&self) -> String {
        self.path().to_string_lossy().to_string()
    }

    pub fn extension(&self) -> String {
        self.path()
            .extension()
            .and_then(|v| v.to_str())
            .unwrap_or("")
            .to_lowercase()
    }
}
