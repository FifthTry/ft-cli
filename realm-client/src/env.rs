pub fn host() -> String {
    if crate::is_test() {
        return "http://127.0.0.1:3000".to_string();
    }
    match std::env::var("REALM_HOST") {
        Ok(host) => host,
        Err(_) => "https://www.fifthtry.com".to_string(),
    }
}
