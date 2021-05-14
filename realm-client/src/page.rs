fn to_url_with_query<K, V>(
    url_: &str,
    query: std::collections::HashMap<K, V>,
) -> crate::Result<url::Url>
where
    K: Into<String> + AsRef<str>,
    V: Into<String> + AsRef<str>,
{
    // TODO: read domain from config/env
    // TODO: ensure the keys are traversed in sorted order
    let params: Vec<(_, _)> = query.iter().collect();
    url::Url::parse_with_params(
        &format!("http://127.0.0.1:3000{}?realm_mode=api", url_),
        &params,
    )
    .map_err(crate::Error::UrlParseError)
}

// TODO: convert it to a macro so key values can be passed easily
pub fn page<T, K, V>(
    url: &str,
    query: std::collections::HashMap<K, V>,
    tid: Option<String>,
) -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
    K: Into<String> + AsRef<str>,
    V: Into<String> + AsRef<str>,
{
    let url = to_url_with_query(url, query)?;

    if crate::is_test() {
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
        Err(e) => return Err(crate::Error::HttpError(e)),
    };

    crate::action::handle_response(resp)
}
