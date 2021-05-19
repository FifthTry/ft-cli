pub type Result<T> = anyhow::Result<T>;

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

#[allow(clippy::upper_case_acronyms)]
pub enum Backend {
    Unknown,
    FTD,
    RAW,
}

impl Backend {
    pub fn accept(&self, path: &std::path::Path) -> bool {
        match self {
            Backend::FTD => path.extension() == Some(std::ffi::OsStr::new("ftd")),
            Backend::RAW => true,
            Backend::Unknown => false,
        }
    }
}

impl From<&str> for Backend {
    fn from(s: &str) -> Backend {
        match s {
            "ftd" => Backend::FTD,
            "raw" => Backend::RAW,
            _ => Backend::Unknown,
        }
    }
}

impl From<&std::string::String> for Backend {
    fn from(s: &std::string::String) -> Backend {
        s.as_str().into()
    }
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Backend::FTD => write!(f, "FTD"),
            Backend::RAW => write!(f, "RAW"),
            Backend::Unknown => write!(f, "Unknown"),
        }
    }
}

pub enum SyncMode {
    LocalToRemote,
    RemoteToLocal,
    TwoWay,
}
