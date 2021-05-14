pub fn action<T, B>(url: &str, body: B, tid: Option<String>) -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
    B: serde::Serialize,
{
    let url = crate::url(url);

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

    crate::handle(crate::client(url.as_str(), reqwest::Method::POST).body(json))
}
