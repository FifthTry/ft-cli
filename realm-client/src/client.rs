pub(crate) fn client(url: &str, method: reqwest::Method) -> reqwest::blocking::RequestBuilder {
    reqwest::blocking::Client::new()
        .request(method, url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", "rust")
}

pub(crate) fn url(u: &str) -> String {
    let host = match std::env::var("FT_HOST") {
        Ok(host) => host,
        Err(_) => "https://www.fifthtry.com".to_string(),
    };
    format!("{}{}?realm_mode=api", host, u)
}

pub(crate) fn handle<T>(req: reqwest::blocking::RequestBuilder) -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let resp = match req.send() {
        Ok(response) => response,
        Err(e) => return Err(crate::Error::HttpError(e)),
    };

    if resp.status() != reqwest::StatusCode::OK {
        return Err(crate::Error::UnexpectedResponse {
            code: resp.status().as_u16(),
            body: resp
                .text()
                .unwrap_or_else(|_| "failed to read body".to_string()),
        });
    };

    match resp.json::<crate::types::ApiResponse<T>>() {
        Ok(v) => match v.0 {
            Ok(v) => Ok(v),
            Err(e) => Err(crate::Error::FieldError(e)),
        },
        Err(err) => Err(crate::Error::DeserializeError(err)),
    }
}
