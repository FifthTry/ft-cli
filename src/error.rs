#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cannot parse config file {error:?}")]
    ConfigFileFTDError { error: ftd::p1::Error },

    #[error("cannot parse config file {error:?}")]
    ConfigFileParseError { error: String },

    #[error("RealmError: {error:?}")]
    RealmError { error: realm_client::Error },

    #[error("Utf8Error: {error:?}")]
    Utf8Error { error: std::string::FromUtf8Error },

    #[error("IOError: {error:?}")]
    IOError { error: std::io::Error },

    #[error("cannot read file: {}, {}", _0, _1)]
    ReadError(std::io::Error, String),

    #[error("api status code: {}", _0)]
    APIResponseNotOk(String),

    #[error("DeserializeError: {}", _0)]
    DeserializeError(String),

    #[error("ResponseError: {}", _0)]
    ResponseError(String),

    #[error("UnknownError: {}", _0)]
    UnknownError(String),
}

impl From<realm_client::Error> for Error {
    fn from(e: realm_client::Error) -> Self {
        Error::RealmError { error: e }
    }
}

impl From<ftd::p1::Error> for Error {
    fn from(e: ftd::p1::Error) -> Self {
        Error::ConfigFileFTDError { error: e }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError { error: e }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error { error: e }
    }
}
