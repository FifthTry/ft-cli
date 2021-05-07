use crate::types::FTResult;

pub fn status(file_name: &str) -> FTResult<()> {
    let config = crate::config::Config::from_file(file_name)?;
    status_util(config, file_name)?;
    Ok(())
}

pub fn status_util(config: crate::config::Config, config_file_path: &str) -> FTResult<()> {
    use crate::types::Auth;
    /*
    Config: ../.ft-sync.p1
    Backend: mdBook
    Root: docs
    Last Sync On: 2021-04-21 3:05PM (CST).
    */

    let authcode = match config.auth {
        Auth::AuthCode(s) => s,
        _ => return Ok(()),
    };

    let (synced_hash, updated_on) = ft_api::status::call(authcode.as_str())?;

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

pub fn sync(file_name: &str, dry_run: bool) -> FTResult<()> {
    let config = crate::config::Config::from_file(file_name)?;
    sync_util(config, dry_run)?;
    Ok(())
}

fn sync_util(config: crate::config::Config, _dry_run: bool) -> FTResult<()> {
    use crate::types::Auth;
    use std::process::Command;

    let authcode = match &config.auth {
        Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let (synced_hash, _) = ft_api::status::call(authcode.as_str())?;

    let output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;
    let latest_hash = String::from_utf8(output.stdout)?;

    let root_dir_output = Command::new("git")
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
        crate::git::git_ls_tree(&latest_hash)?
    } else {
        crate::git::git_diff(&synced_hash, &latest_hash)?
    };

    let mut actions = vec![];
    let read_content = |file_path: &str| -> FTResult<String> {
        std::fs::read_to_string(&file_path)
            .map_err(|e| crate::error::FTSyncError::ReadError(e).into())
    };

    for file in files.into_iter() {
        match file {
            crate::git::FileMode::Added(path) => {
                let path = std::path::Path::new(root_dir).join(path);
                if config.backend.accept(&path) {
                    if let Some(path) = path.to_str() {
                        let new_path = path.replacen(&data_dir, "", 1).replacen(".ftd", "", 1);
                        println!("path: {}, new_path: {}", path, new_path);
                        actions.push(ft_api::Action::Added {
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
                        actions.push(ft_api::Action::Added {
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
                        actions.push(ft_api::Action::Deleted { id: new_path });
                    }
                }
            }
            crate::git::FileMode::Modified(path) => {
                let path = std::path::Path::new(root_dir).join(path);
                if config.backend.accept(&path) {
                    if let Some(path) = path.to_str() {
                        let new_path = path.replacen(&data_dir, "", 1).replacen(".ftd", "", 1);
                        println!("path: {}, new_path: {}", path, new_path);
                        actions.push(ft_api::Action::Added {
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
                        actions.push(ft_api::Action::Deleted { id: new_path });
                    }
                }
            }
        }
    }

    println!("files {:#?}", actions);

    ft_api::bulk_update::call(
        config.collection.as_str(),
        synced_hash.as_str(),
        latest_hash.as_str(),
        config.repo.as_str(),
        actions,
        authcode.as_str(),
    )?;

    Ok(())
}
