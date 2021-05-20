pub fn status(config: &crate::Config) -> crate::Result<()> {
    let auth_code = match config.auth {
        crate::Auth::AuthCode(ref s) => s,
        _ => return Ok(()),
    };

    let status = ft_api::sync_status(
        config.collection.as_str(),
        auth_code.as_str(),
        &crate::utils::platform()?,
        &crate::utils::client_version(),
    )?;

    // println!("Config: {}", config_file);
    // println!("Backend: {}", config.backend.to_string());
    println!("Root: {}", config.root);
    println!(
        "Last git synced hash: {}",
        if status.last_synced_hash.is_empty() {
            "Never synced"
        } else {
            status.last_synced_hash.as_str()
        }
    );

    if crate::is_test() {
        // we fix the timezone to IST in test mode so on github etc we get consistent output
        // let local: chrono::DateTime<chrono_tz::Asia::Kolkata> =
        //     chrono::DateTime::from(status.last_updated_on);
        let last_updated_on_in_ist = status
            .last_updated_on
            .with_timezone(&chrono_tz::Asia::Kolkata);
        println!("Last synced on: {:?}", last_updated_on_in_ist);
    } else {
        let local: chrono::DateTime<chrono::Local> = chrono::DateTime::from(status.last_updated_on);
        println!("Last synced on: {:?}", local);
    }

    Ok(())
}
