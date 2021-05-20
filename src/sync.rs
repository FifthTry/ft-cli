pub fn sync(config: &crate::Config, _dry_run: bool) -> crate::Result<()> {
    let auth_code = match &config.auth {
        crate::Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let latest_hash = crate::git::head()?;
    let git_root = crate::git::root_dir()?;

    let root_dir = {
        let root_dir = config.root_abs_path();
        if !root_dir.starts_with(&git_root) {
            panic!(
                "The root directory: {:?} is not inside git dir: {}",
                root_dir.as_os_str(),
                &git_root
            )
        }

        root_dir.to_string_lossy().to_string()
    };

    let status = ft_api::sync_status(
        config.collection.as_str(),
        auth_code.as_str(),
        &crate::utils::platform()?,
        &crate::utils::client_version(),
    )?;

    let actions = {
        let mut actions = vec![];

        let files = if status.last_synced_hash.is_empty() {
            crate::git::ls_tree(&latest_hash, &git_root, &root_dir)?
        } else {
            crate::git::changed_files(&status.last_synced_hash, &latest_hash, &git_root, &root_dir)?
        };

        for file in files.into_iter() {
            actions.append(&mut match config.backend {
                crate::Backend::FTD => {
                    crate::ftd::handle(file, root_dir.as_str(), config.collection.as_str())?
                }
                crate::Backend::Raw => {
                    crate::raw::handle(file, root_dir.as_str(), config.collection.as_str())?
                }
            });
        }

        actions
    };

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
