use thiserror::Error;

#[derive(Error, Debug)]
pub enum FTSyncError {
    #[error("cannot open config file: {error:?}")]
    ConfigFileReadError { error: std::io::Error },

    #[error("cannot parse config file {error:?}")]
    ConfigFileFTDError { error: ftd::document::ParseError },

    #[error("cannot parse config file {error:?}")]
    ConfigFileParseError { error: String },

    #[error("api error: {error:?}")]
    APIError { error: reqwest::Error },
}

impl From<reqwest::Error> for FTSyncError {
    fn from(e: reqwest::Error) -> Self {
        FTSyncError::APIError { error: e }
    }
}

impl From<std::io::Error> for FTSyncError {
    fn from(e: std::io::Error) -> Self {
        FTSyncError::ConfigFileReadError { error: e }
    }
}

impl From<ftd::document::ParseError> for FTSyncError {
    fn from(e: ftd::document::ParseError) -> Self {
        FTSyncError::ConfigFileFTDError { error: e }
    }
}
