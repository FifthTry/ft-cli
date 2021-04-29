use gitignore;
use std::fmt::Error;

pub struct Config<'a> {
    ignored: Vec<gitignore::Pattern<'a>>,
    repo: String,
    collection: String,
    backend: Backend,
    root: String,
    mode: SyncMode,
    auth: Auth,
    dot_ft: bool,
}

enum Auth {
    SignedIn(User),
    AuthCode(String),
    Anonymous,
}

struct User {
    cookie: String,
    username: String,
    name: String,
}

enum Backend {
    FTD,
}

enum SyncMode {
    LocalToRemote,
    RemoteToLocal,
    TwoWay,
}
