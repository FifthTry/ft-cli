fn to_url_with_query<K, V>(
    u: &str,
    query: std::collections::HashMap<K, V>,
) -> crate::Result<url::Url>
where
    K: Into<String> + AsRef<str> + Ord,
    V: Into<String> + AsRef<str>,
{
    let mut params: Vec<(_, _)> = query.iter().collect();
    params.sort_by(|(a, _), (b, _)| a.cmp(b));

    url::Url::parse_with_params(crate::url(u).as_str(), &params)
        .map_err(crate::Error::UrlParseError)
}

pub fn page<T, K, V>(
    url: &str,
    query: std::collections::HashMap<K, V>,
    tid: Option<String>,
) -> crate::Result<T>
where
    T: serde::de::DeserializeOwned,
    K: Into<String> + AsRef<str> + Ord,
    V: Into<String> + AsRef<str>,
{
    let url = to_url_with_query(url, query)?;

    if crate::is_test() {
        return crate::mock(tid, serde_json::json! ({"url": url.as_str()}));
    }

    crate::handle(crate::client(url.as_str(), reqwest::Method::GET))
}
