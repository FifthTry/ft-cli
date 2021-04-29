use gitignore;
use std;
use std::convert::Into;

pub struct Config<'a> {
    pub ignored: Vec<gitignore::Pattern<'a>>,
    pub repo: String,
    pub collection: String,
    pub backend: Backend,
    pub root: &'a std::path::Path,
    pub mode: SyncMode,
    pub auth: Auth,
    pub dot_ft: bool,
}

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

impl From<&str> for Backend {
    fn from(s: &str) -> Backend {
        match s {
            "ftd" => Backend::FTD,
            _ => Backend::Unknown,
        }
    }
}

pub enum SyncMode {
    LocalToRemote,
    RemoteToLocal,
    TwoWay,
}
