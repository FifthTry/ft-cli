#[derive(Debug)]
pub(crate) struct ApiResponse<T: serde::de::DeserializeOwned>(
    pub std::result::Result<T, std::collections::HashMap<String, String>>,
);

#[derive(serde_derive::Deserialize)]
struct A<T> {
    pub success: bool,
    pub result: Option<T>,
    // TODO: change to `pub error: std::collections::HashMap<String, String>,`
    pub error: Option<std::collections::HashMap<String, String>>,
}

impl<'de, T> serde::de::Deserialize<'de> for ApiResponse<T>
where
    T: serde::de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let a = A::deserialize(deserializer)?;
        if a.success {
            match a.result {
                Some(v) => Ok(ApiResponse(Ok(v))),
                None => Err(serde::de::Error::custom(
                    "success is true but result is None",
                )),
            }
        } else {
            match a.error {
                Some(v) => Ok(ApiResponse(Err(v))),
                None => Err(serde::de::Error::custom(
                    "success is false but error is None",
                )),
            }
        }
    }
}

pub fn never_expected<'de, D, T>(_deserializer: D) -> std::result::Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    unreachable!("must never happen")
}

#[derive(Debug, thiserror::Error, serde_derive::Deserialize)]
pub enum Error {
    #[serde(deserialize_with = "never_expected")]
    #[error("HttpError: {}", _0)]
    HttpError(reqwest::Error),
    #[error("UnexpectedResponse: {code:?} {body:?}")]
    UnexpectedResponse {
        // non 200
        body: String,
        code: u16,
    },
    // SpecificError(T),
    #[error("PageNotFound: {}", _0)]
    PageNotFound(String),
    #[error("FieldError: {:?}", _0)]
    FieldError(std::collections::HashMap<String, String>), // How to make realm return this?
    #[serde(deserialize_with = "never_expected")]
    #[error("DeserializeError: {:?}", _0)]
    DeserializeError(reqwest::Error),
    #[serde(deserialize_with = "never_expected")]
    #[error("SerializeError: {:?}", _0)]
    SerializeError(serde_json::Error),
    #[serde(deserialize_with = "never_expected")]
    #[error("UrlParseError: {:?}", _0)]
    UrlParseError(url::ParseError),
    #[serde(deserialize_with = "never_expected")]
    #[error("SerdeDeserializeError: {:?}", _0)]
    SerdeDeserializeError(serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::UrlParseError(e)
    }
}

// #[derive(thiserror::Error, Debug)]
// pub enum ActionError {
//     #[error("api error: {error:?}")]
//     APIError { error: reqwest::Error },
//
//     #[error("api status code: {}", _0)]
//     APIResponseNotOk(String),
//
//     #[error("DeserializeError: {}", _0)]
//     DeserializeError(String),
//
//     #[error("ResponseError: {}", _0)]
//     ResponseError(String),
//
//     #[error("PageError: {:?}", _0)]
//     PageError(crate::Error),
// }
//
// impl From<reqwest::Error> for ActionError {
//     fn from(e: reqwest::Error) -> Self {
//         Self::APIError { error: e }
//     }
// }
//
// impl From<crate::Error> for ActionError {
//     fn from(e: crate::Error) -> Self {
//         Self::PageError(e)
//     }
// }
