pub fn action<T, B>(url: &str, body: B, tid: Option<String>) -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
    B: serde::Serialize,
{
    let url = crate::url(url);

    if crate::is_test() {
        return crate::mock(tid, serde_json::json! ({"url": url.as_str(), "body": body}));
    }

    let json = match serde_json::to_string(&body) {
        Ok(v) => v,
        Err(e) => return Err(crate::Error::SerializeError(e)),
    };

    crate::handle(crate::client(url.as_str(), reqwest::Method::POST).body(json))
}
