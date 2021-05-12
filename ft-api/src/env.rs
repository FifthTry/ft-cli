pub fn host() -> String {
    match std::env::var("FT_HOST") {
        Ok(host) => host,
        Err(_) => "http://127.0.0.1:3000".to_string(),
    }
}
