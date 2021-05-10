#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub result: Option<T>,
    // TODO: change to `pub error: std::collections::HashMap<String, String>,`
    pub error: Option<ApiError>,
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error: String,
}

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}

#[derive(Deserialize, Debug)]
pub enum Error {
    InvalidAuthCode,
    RepoNotFound,
    CollectionNotFound,
    InvalidID,
    HashNotMatching,
    InvalidFileName(String),
    BadFTD(String),
    NoPermission(String),
    DBError,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::InvalidAuthCode => "InvalidAuthCode".to_string(),
            Error::RepoNotFound => "RepoNotFound".to_string(),
            Error::CollectionNotFound => "CollectionNotFound".to_string(),
            Error::InvalidID => "InvalidID".to_string(),
            Error::HashNotMatching => "HashNotMatching".to_string(),
            Error::InvalidFileName(name) => format!("InvalidFileName: {}", name),
            Error::BadFTD(s) => format!("BadFTD: {}", s),
            Error::NoPermission(p) => format!("NoPermission: {}", p),
            Error::DBError => "DBError".to_string(),
        }
    }
}

fn to_url_with_query<K, V>(url: &str, _query: std::collections::HashMap<K, V>) -> String
where
    K: Into<String>,
    V: Into<String>,
{
    // TODO: read domain from config/env
    // TODO: ensure the keys are traversed in sorted order
    format!("http://127.0.0.1:3000{}?realm_mode=api", url)
}

fn to_url(url: &str) -> String {
    // TODO: read domain from config/env
    format!("http://127.0.0.1:3000{}?realm_mode=api", url)
}

#[derive(Debug, thiserror::Error)]
pub enum PageError {
    #[error("HttpError: {}", _0)]
    HttpError(reqwest::Error),
    #[error("UnexpectedResponse: {code:?} {body:?}")]
    UnexpectedResponse {
        // non 200
        body: String,
        code: reqwest::StatusCode,
    },
    #[error("PageNotFound: {}", _0)]
    PageNotFound(String),
    #[error("InputError: {:?}", _0)]
    InputError(std::collections::HashMap<String, String>), // How to make realm return this?
    #[error("DeserializeError: {:?}", _0)]
    DeserializeError(reqwest::Error),
}

pub type PageResult<T> = Result<T, PageError>;

// TODO: convert it to a macro so key values can be passed easily
pub fn page<T, K, V>(
    url: &str,
    query: std::collections::HashMap<K, V>,
    tid: Option<String>,
) -> PageResult<T>
where
    T: serde::de::DeserializeOwned,
    K: Into<String>,
    V: Into<String>,
{
    let url = to_url_with_query(url, query);

    if is_test() {
        let tid = match tid {
            Some(v) => v,
            None => panic!("tid is none in test mode"),
        };

        // write to ./tid.url and return content of tid.json
        std::fs::write(format!("{}.url", tid.as_str()), url).expect("failed to write to .url file");
        return Ok(serde_json::from_str(
            std::fs::read_to_string(format!("{}.json", tid.as_str()))
                .expect("failed to read .json file")
                .as_str(),
        )
        .expect("failed to parse json"));
    }

    let client = reqwest::blocking::Client::new();
    let resp = match client
        .get(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", "rust")
        .send()
    {
        Ok(response) => response,
        Err(e) => return Err(PageError::HttpError(e)),
    };

    if resp.status() != reqwest::StatusCode::OK {
        // TODO: handle 404 and input errors
        return Err(PageError::UnexpectedResponse {
            code: resp.status(),
            body: resp
                .text()
                .unwrap_or_else(|_| "failed to read body".to_string()),
        });
    }

    resp.json().map_err(PageError::DeserializeError)
}

pub fn action<T, B>(url: &str, body: B, tid: Option<String>) -> crate::Result<ApiResponse<T>>
where
    T: serde::de::DeserializeOwned,
    B: serde::Serialize,
{
    let url = to_url(url);

    if is_test() {
        let tid = match tid {
            Some(v) => v,
            None => panic!("tid is none in test mode"),
        };

        // write to ./tid.url and return content of tid.json
        std::fs::write(format!("{}.url", tid.as_str()), url).expect("failed to write to .url file");
        std::fs::write(
            format!("{}.out.json", tid.as_str()),
            sorted_json::to_json(&serde_json::to_value(body).expect("failed to serialise"))
                .as_str(),
        )
        .expect("failed to write to .out.json file");
        return Ok(serde_json::from_str(
            std::fs::read_to_string(format!("{}.json", tid.as_str()))
                .expect("failed to read .json file")
                .as_str(),
        )
        .expect("failed to parse json"));
    }

    let client = reqwest::blocking::Client::new();
    let resp = match client
        .post(to_url(url.as_str()))
        .body(reqwest::blocking::Body::from(serde_json::to_vec(&body)?))
        .header("content-type", "application/json")
        .header("Accept", "application/json")
        .header("user-agent", "rust")
        .send()
    {
        Ok(response) => response,
        Err(e) => return Err(crate::error::Error::APIError { error: e }.into()),
    };

    if resp.status() != reqwest::StatusCode::OK {
        return Err(
            crate::error::Error::APIResponseNotOk("post api response not OK".to_string()).into(),
        );
    };

    resp.json().map_err(Into::into)
}
