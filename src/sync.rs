pub fn sync(config: &crate::Config) -> crate::Result<()> {
    let auth_code = match &config.auth {
        crate::Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let latest_hash = crate::git::head()?;

    let status = ft_api::sync_status(
        config.collection.as_str(),
        auth_code.as_str(),
        &crate::utils::platform()?,
        &crate::utils::client_version(),
    )?;

    let actions = {
        let files = if status.last_synced_hash.is_empty() {
            crate::git::ls_tree(&latest_hash, config.root.as_str())?
        } else {
            crate::git::changed_files(&status.last_synced_hash, &latest_hash, config.root.as_str())?
        };

        match config.backend {
            crate::Backend::FTD => crate::ftd::handle_files(config, &files)?,
            crate::Backend::Raw => crate::raw::handle_files(config, &files)?,
            crate::Backend::MdBook => crate::mdbook::handle_files(config, &files)?,
        }
    };

    // println!("{:#?}", actions);

    let st = std::time::Instant::now();

    ft_api::bulk_update(
        config.collection.as_str(),
        status.last_synced_hash.as_str(),
        latest_hash.as_str(),
        config.repo.as_str(),
        actions,
        auth_code.as_str(),
        crate::utils::platform()?,
        crate::utils::client_version(),
    )?;

    if crate::is_test() {
        println!("Synced successfully.");
    } else {
        println!("Synced successfully: {}.", crate::utils::elapsed(st));
    }

    Ok(())
}
