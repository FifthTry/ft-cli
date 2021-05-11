#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub result: Option<T>,
    // TODO: change to `pub error: std::collections::HashMap<String, String>,`
    pub error: Option<std::collections::HashMap<String, String>>,
}

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}

fn to_url_with_query<K, V>(
    url_: &str,
    query: std::collections::HashMap<K, V>,
) -> PageResult<url::Url>
where
    K: Into<String> + AsRef<str>,
    V: Into<String> + AsRef<str>,
{
    use std::iter::FromIterator;
    // TODO: read domain from config/env
    // TODO: ensure the keys are traversed in sorted order
    let params: Vec<(_, _)> = Vec::from_iter(query.iter());
    url::Url::parse_with_params(
        &format!("http://127.0.0.1:3000{}?realm_mode=api", url_),
        &params,
    )
    .map_err(PageError::UrlParseError)
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
    #[error("UrlParseError: {:?}", _0)]
    UrlParseError(url::ParseError),
    #[error("SerdeDeserializeError: {:?}", _0)]
    SerdeDeserializeError(serde_json::Error),
}

impl From<url::ParseError> for PageError {
    fn from(e: url::ParseError) -> Self {
        Self::UrlParseError(e)
    }
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
    K: Into<String> + AsRef<str>,
    V: Into<String> + AsRef<str>,
{
    let url = to_url_with_query(url, query)?;

    if is_test() {
        let tid = match tid {
            Some(v) => v,
            None => panic!("tid is none in test mode"),
        };

        // write to ./tid.url and return content of tid.json
        std::fs::write(format!("{}.url", tid.as_str()), url.as_str())
            .expect("failed to write to .url file");
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

    let status = resp.status();

    let resp_value: Result<ApiResponse<serde_json::Value>, reqwest::Error> = resp.json();
    match resp_value {
        Ok(r) => {
            if !r.success {
                return Err(PageError::UnexpectedResponse {
                    code: status,
                    body: r.error.map_or("Something went wrong".to_string(), |x| {
                        x.into_iter()
                            .map(|(k, v)| k + ": " + &v)
                            .collect::<Vec<_>>()
                            .join("\n")
                    }),
                });
            }
            match r.result {
                Some(v) => serde_json::from_value(v).map_err(PageError::SerdeDeserializeError),
                None => {
                    return Err(PageError::UnexpectedResponse {
                        code: status,
                        body: "Response is not present".to_string(),
                    })
                }
            }
        }
        Err(err) => {
            return Err(PageError::UnexpectedResponse {
                code: status,
                body: err.to_string(),
            })
        }
    }
}

pub fn action<T, B>(url: &str, body: B, tid: Option<String>) -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
    B: Into<reqwest::blocking::Body> + serde::Serialize,
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
        .post(url.as_str())
        .body(body)
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

    let resp_value: Result<ApiResponse<serde_json::Value>, reqwest::Error> = resp.json();

    match resp_value {
        Ok(v) => {
            if !v.success {
                return Err(crate::error::Error::ResponseError(v.error.map_or(
                    "Something went wrong".to_string(),
                    |x| {
                        x.into_iter()
                            .map(|(k, v)| k + ": " + &v)
                            .collect::<Vec<_>>()
                            .join("\n")
                    },
                ))
                .into());
            }

            match v.result {
                Some(v) => serde_json::from_value(v)
                    .map_err(|e| crate::error::Error::DeserializeError(e.to_string()).into()),
                None => return Err(crate::error::Error::APIResponseNotOk("".to_string()).into()),
            }
        }
        Err(err) => return Err(crate::error::Error::ResponseError(err.to_string()).into()),
    }
}
