pub fn status(file_name: &str) -> crate::Result<()> {
    let config = crate::Config::from_file(file_name)?;
    status_util(config, file_name)?;
    Ok(())
}

pub fn status_util(config: crate::Config, config_file_path: &str) -> crate::Result<()> {
    /*
    Config: ../.ft-sync.p1
    Backend: mdBook
    Root: docs
    Last Sync On: 2021-04-21 3:05PM (CST).
    */

    let auth_code = match config.auth {
        crate::Auth::AuthCode(s) => s,
        _ => return Ok(()),
    };

    let (synced_hash, updated_on) = ft_api::sync_status(auth_code.as_str())?;

    println!("Config: {}", config_file_path);
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
    println!("Last Sync On: {}", updated_on.to_rfc3339());

    Ok(())
}

pub fn sync(file_name: &str, dry_run: bool) -> crate::Result<()> {
    let config = crate::Config::from_file(file_name)?;
    sync_util(config, dry_run)?;
    Ok(())
}

fn sync_util(config: crate::Config, _dry_run: bool) -> crate::Result<()> {
    let auth_code = match &config.auth {
        crate::Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let (synced_hash, _) = ft_api::sync_status::sync_status(auth_code.as_str())?;

    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()?;
    let latest_hash = String::from_utf8(output.stdout)?;

    let root_dir_output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;
    let root_dir = String::from_utf8(root_dir_output.stdout)?;
    let root_dir = root_dir.trim();

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
                let path = std::path::Path::new(root_dir).join(path);
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
                let path = std::path::Path::new(root_dir).join(p2);
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

                let path = std::path::Path::new(root_dir).join(p1);
                if config.backend.accept(&path) {
                    if let Some(path) = path.to_str() {
                        let new_path = path.replacen(&data_dir, "", 1).replacen(".ftd", "", 1);
                        println!("path: {}, new_path: {}", path, new_path);
                        actions.push(ft_api::bulk_update::Action::Deleted { id: new_path });
                    }
                }
            }
            crate::git::FileMode::Modified(path) => {
                let path = std::path::Path::new(root_dir).join(path);
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
                let path = std::path::Path::new(root_dir).join(path);
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
