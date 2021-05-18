pub(crate) fn platform() -> crate::Result<String> {
    if crate::is_test() {
        return Ok("test-platform".to_string());
    }

    let output = std::process::Command::new("uname").arg("-a").output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub(crate) fn client_version() -> String {
    if crate::is_test() {
        return "test-version".to_string();
    }
    env!("CARGO_PKG_VERSION").to_string()
}
