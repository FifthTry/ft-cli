pub fn auth_code() -> String {
    match std::env::var("FT_AUTH_CODE") {
        Ok(code) => code,
        Err(_) => panic!("FT_AUTH_CODE not found in environment"),
    }
}
