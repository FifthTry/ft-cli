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
        let read_content = |file_path: &str| -> crate::Result<String> {
            std::fs::read_to_string(&file_path)
                .map_err(|e| crate::Error::ReadError(e, file_path.to_string()).into())
        };

        let to_docid = |path: &str| -> String {
            let t = std::path::Path::new(&path)
                .strip_prefix(root_dir.as_str())
                .unwrap()
                .with_extension("")
                .to_str()
                .unwrap()
                .to_string();
            if t == "index" {
                config.collection.to_string()
            } else {
                config.collection.to_string() + "/" + t.as_str()
            }
        };

        let mut actions = vec![];

        let files = if status.last_synced_hash.is_empty() {
            crate::git::ls_tree(&latest_hash, &git_root, &root_dir)?
        } else {
            crate::git::changed_files(&status.last_synced_hash, &latest_hash, &git_root, &root_dir)?
        };

        for file in files.into_iter() {
            match file {
                crate::git::FileMode::Added(path) => {
                    if config.backend.accept(std::path::Path::new(&path)) {
                        let docid = to_docid(&path);
                        println!("Added new: {}", path);
                        actions.push(ft_api::bulk_update::Action::Added {
                            id: docid,
                            content: read_content(&path)?,
                        });
                    }
                }

                crate::git::FileMode::Modified(path) => {
                    if config.backend.accept(std::path::Path::new(&path)) {
                        let docid = to_docid(&path);
                        println!("Updated: {}", path);
                        actions.push(ft_api::bulk_update::Action::Updated {
                            id: docid,
                            content: read_content(&path)?,
                        });
                    }
                }
                crate::git::FileMode::Deleted(path) => {
                    if config.backend.accept(std::path::Path::new(&path)) {
                        let docid = to_docid(&path);
                        println!("Deleted: {}", path);
                        actions.push(ft_api::bulk_update::Action::Deleted { id: docid });
                    }
                }
            }
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

    println!("Synced successfully: {}", crate::utils::elapsed(st));

    Ok(())
}
