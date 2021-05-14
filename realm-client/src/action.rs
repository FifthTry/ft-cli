fn to_url(url: &str) -> String {
    // TODO: read domain from config/env
    format!("http://127.0.0.1:3000{}?realm_mode=api", url)
}

pub fn action<T, B>(url: &str, body: B, tid: Option<String>) -> crate::Result<T>
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

    let json = match serde_json::to_string(&body) {
        Ok(v) => v,
        Err(e) => return Err(crate::Error::SerializeError(e)),
    };

    let client = reqwest::blocking::Client::new();
    let resp = match client
        .post(url.as_str())
        .body(json)
        .header("content-type", "application/json")
        .header("Accept", "application/json")
        .header("user-agent", "rust")
        .send()
    {
        Ok(response) => response,
        Err(e) => return Err(crate::Error::HttpError(e)),
    };

    handle_response(resp)
}

pub(crate) fn handle_response<T>(resp: reqwest::blocking::Response) -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    if resp.status() != reqwest::StatusCode::OK {
        return Err(crate::Error::UnexpectedResponse {
            code: resp.status(),
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
