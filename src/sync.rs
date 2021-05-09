pub fn sync(config: &crate::Config, _dry_run: bool) -> crate::Result<()> {
    let auth_code = match &config.auth {
        crate::Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let latest_hash = crate::git::head()?;
    let root_dir = crate::git::root_dir()?;

    let (synced_hash, _) =
        ft_api::sync_status::sync_status(config.collection.as_str(), auth_code.as_str())?;

    let data_dir = std::path::Path::new(&root_dir).join(&config.root);

    let data_dir = match data_dir.to_str() {
        Some(s) => s.to_string() + "/",
        None => "/".to_string(),
    };

    let files = if synced_hash.is_empty() {
        crate::git::ls_tree(&latest_hash)?
    } else {
        crate::git::diff(&synced_hash, &latest_hash)?
    };

    let mut actions = vec![];
    let read_content = |file_path: &str| -> crate::Result<String> {
        std::fs::read_to_string(&file_path).map_err(|e| crate::Error::ReadError(e).into())
    };

    for file in files.into_iter() {
        match file {
            crate::git::FileMode::Added(path) => {
                let path = std::path::Path::new(&root_dir).join(path);
                if config.backend.accept(&path) {
                    if let Some(path) = path.to_str() {
                        let new_path = path.replacen(&data_dir, "", 1).replacen(".ftd", "", 1);
                        println!("path: {}, new_path: {}", path, new_path);
                        actions.push(ft_api::bulk_update::Action::Added {
                            id: new_path,
                            content: read_content(path)?,
                        });
                    }
                }
            }
            crate::git::FileMode::Renamed(p1, p2) => {
                let path = std::path::Path::new(&root_dir).join(p2);
                if config.backend.accept(&path) {
                    if let Some(path) = path.to_str() {
                        let new_path = path.replacen(&data_dir, "", 1).replacen(".ftd", "", 1);
                        println!("path: {}, new_path: {}", path, new_path);
                        actions.push(ft_api::bulk_update::Action::Added {
                            id: new_path,
                            content: read_content(path)?,
                        });
                    }
                }

                let path = std::path::Path::new(&root_dir).join(p1);
                if config.backend.accept(&path) {
                    if let Some(path) = path.to_str() {
                        let new_path = path.replacen(&data_dir, "", 1).replacen(".ftd", "", 1);
                        println!("path: {}, new_path: {}", path, new_path);
                        actions.push(ft_api::bulk_update::Action::Deleted { id: new_path });
                    }
                }
            }
            crate::git::FileMode::Modified(path) => {
                let path = std::path::Path::new(&root_dir).join(path);
                if config.backend.accept(&path) {
                    if let Some(path) = path.to_str() {
                        let new_path = path.replacen(&data_dir, "", 1).replacen(".ftd", "", 1);
                        println!("path: {}, new_path: {}", path, new_path);
                        actions.push(ft_api::bulk_update::Action::Updated {
                            id: new_path,
                            content: read_content(path)?,
                        });
                    }
                }
            }
            crate::git::FileMode::Deleted(path) => {
                let path = std::path::Path::new(&root_dir).join(path);
                if config.backend.accept(&path) {
                    if let Some(path) = path.to_str() {
                        let new_path = path.replacen(&data_dir, "", 1).replacen(".ftd", "", 1);
                        println!("path: {}, new_path: {}", path, new_path);
                        actions.push(ft_api::bulk_update::Action::Deleted { id: new_path });
                    }
                }
            }
        }
    }

    println!("files {:#?}", actions);

    ft_api::bulk_update::bulk_update(
        config.collection.as_str(),
        synced_hash.as_str(),
        latest_hash.as_str(),
        config.repo.as_str(),
        actions,
        auth_code.as_str(),
    )?;

    Ok(())
}
