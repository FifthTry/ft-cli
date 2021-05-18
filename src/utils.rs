pub(crate) fn platform() -> crate::Result<String> {
    let output = std::process::Command::new("uname").arg("-a").output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub(crate) fn client_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
