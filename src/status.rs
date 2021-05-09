pub fn status(config: &crate::Config, config_file: &str) -> crate::Result<()> {
    /*
    Config: ../.ft-sync.p1
    Backend: mdBook
    Root: docs
    Last Sync On: 2021-04-21 3:05PM (CST).
    */

    let auth_code = match config.auth {
        crate::Auth::AuthCode(ref s) => s,
        _ => return Ok(()),
    };

    let (synced_hash, updated_on) =
        ft_api::sync_status(config.collection.as_str(), auth_code.as_str())?;

    println!("Config: {}", config_file);
    println!("Backend: {}", config.backend.to_string());
    println!("Root: {}", config.root);
    println!(
        "Last Synced Hash: {}",
        if synced_hash.is_empty() {
            "Never Synced"
        } else {
            synced_hash.as_str()
        }
    );

    let local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(updated_on);
    println!("Last Sync On: {:?}", local);

    Ok(())
}
