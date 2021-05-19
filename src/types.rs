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
    Raw,
}

impl Backend {
    pub fn accept(&self, path: &std::path::Path) -> bool {
        match self {
            Backend::FTD => path.extension() == Some(std::ffi::OsStr::new("ftd")),
            Backend::Raw => true,
            Backend::Unknown => false,
        }
    }

    pub fn pattern(&self) -> Option<String> {
        match self {
            Backend::FTD => Some("**/*.ftd".to_string()),
            Backend::Raw => Some("**/*.*".to_string()),
            Backend::Unknown => None,
        }
    }
}

impl From<&str> for Backend {
    fn from(s: &str) -> Backend {
        match s {
            "ftd" => Backend::FTD,
            "raw" => Backend::Raw,
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
            Backend::Raw => write!(f, "Raw"),
            Backend::Unknown => write!(f, "Unknown"),
        }
    }
}

pub enum SyncMode {
    LocalToRemote,
    RemoteToLocal,
    TwoWay,
}
