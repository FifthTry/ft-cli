use thiserror::Error;

#[derive(Error, Debug)]
pub enum FTSyncError {
    #[error("cannot open config file {file:?}: {error:?}")]
    ConfigFileReadError { file: String, error: String },

    #[error("cannot parse config file {file:?}: {error:?}")]
    ConfigFileParseError { file: String, error: String },
}
