pub fn host() -> String {
    match std::env::var("REALM_HOST") {
        Ok(host) => host,
        Err(_) => "https://www.fifthtry.com".to_string(),
    }
}
