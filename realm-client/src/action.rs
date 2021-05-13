fn to_url(url: &str) -> String {
    // TODO: read domain from config/env
    format!("http://127.0.0.1:3000{}?realm_mode=api", url)
}

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

pub type Result<T> = anyhow::Result<T>;

pub fn action<T, B>(url: &str, body: B, tid: Option<String>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
    B: serde::Serialize,
{
    let url = to_url(url);

    if crate::is_test() {
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
        .body(serde_json::to_string(&body)?)
        .header("content-type", "application/json")
        .header("Accept", "application/json")
        .header("user-agent", "rust")
        .send()
    {
        Ok(response) => response,
        Err(e) => return Err(Error::APIError { error: e }.into()),
    };

    if resp.status() != reqwest::StatusCode::OK {
        return Err(Error::APIResponseNotOk("post api response not OK".to_string()).into());
    };

    let resp_value: std::result::Result<crate::ApiResponse<serde_json::Value>, reqwest::Error> =
        resp.json();

    match resp_value {
        Ok(v) => {
            if !v.success {
                return Err(Error::ResponseError(v.error.map_or(
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
                    .map_err(|e| Error::DeserializeError(e.to_string()).into()),
                None => Err(Error::APIResponseNotOk("".to_string()).into()),
            }
        }
        Err(err) => Err(Error::ResponseError(err.to_string()).into()),
    }
}
