#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("api error: {error:?}")]
    APIError { error: reqwest::Error },

    #[error("api status code: {}", _0)]
    APIResponseNotOk(String),

    #[error("DeserializeError: {}", _0)]
    DeserializeError(String),

    #[error("ResponseError: {}", _0)]
    ResponseError(String),

    #[error("PageError: {:?}", _0)]
    PageError(crate::PageError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::APIError { error: e }
    }
}

impl From<crate::PageError> for Error {
    fn from(e: crate::PageError) -> Self {
        Self::PageError(e)
    }
}
