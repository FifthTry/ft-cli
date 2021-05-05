use std::fmt;

pub type FTResult<T> = anyhow::Result<T>;

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

pub enum Backend {
    Unknown,
    FTD,
}

impl Backend {
    pub fn accept(&self, path: &std::path::Path) -> bool {
        match self {
            Backend::FTD => path.extension() == Some(std::ffi::OsStr::new("ftd")),
            Backend::Unknown => false
        }
    }
}

impl From<&str> for Backend {
    fn from(s: &str) -> Backend {
        match s {
            "ftd" => Backend::FTD,
            _ => Backend::Unknown,
        }
    }
}

impl From<&std::string::String> for Backend {
    fn from(s: &std::string::String) -> Backend {
        s.as_str().into()
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Backend::FTD => write!(f, "FTD"),
            Backend::Unknown => write!(f, "Unknown"),
        }
    }
}

pub enum SyncMode {
    LocalToRemote,
    RemoteToLocal,
    TwoWay,
}
