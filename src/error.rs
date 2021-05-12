#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cannot parse config file {error:?}")]
    ConfigFileFTDError { error: ftd::document::ParseError },

    #[error("cannot parse config file {error:?}")]
    ConfigFileParseError { error: String },

    #[error("api error: {error:?}")]
    APIError { error: reqwest::Error },

    #[error("cannot read file: {}, {}", _0, _1)]
    ReadError(std::io::Error, String),

    #[error("api status code: {}", _0)]
    APIResponseNotOk(String),

    #[error("DeserializeError: {}", _0)]
    DeserializeError(String),

    #[error("ResponseError: {}", _0)]
    ResponseError(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::APIError { error: e }
    }
}

impl From<ftd::document::ParseError> for Error {
    fn from(e: ftd::document::ParseError) -> Self {
        Error::ConfigFileFTDError { error: e }
    }
}
