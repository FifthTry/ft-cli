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
    // println!("{:?}", data_dir);

    fn parse_line(line: &str) -> ft_api::FileMode {
        let sp = line.split("\t").collect::<Vec<_>>();
        let mode = sp[0].chars().next().unwrap();
        match mode {
            'A' => ft_api::FileMode::Added(sp[1].to_string()),
            'M' => ft_api::FileMode::Modified(sp[1].to_string()),
            'D' => ft_api::FileMode::Deleted(sp[1].to_string()),
            'R' => ft_api::FileMode::Renamed(sp[1].to_string(), sp[2].to_string()),
            _ => panic!("file with unknown mode : {}", line),
        }
    }

    let files = if synced_hash.is_empty() {
        let cmd = Command::new("git")
            .args(&["ls-tree", "-r", "--name-only", latest_hash.trim()])
            .output()?;
        let files = String::from_utf8(cmd.stdout.clone())?;
        let files = files.lines();
        files
            .into_iter()
            .map(|x| ft_api::FileMode::Added(x.to_string()))
            .collect()
    } else {
        let cmd = Command::new("git")
            .args(&[
                "diff",
                "--name-status",
                synced_hash.trim(),
                latest_hash.trim(),
            ])
            .output()?;
        let files = String::from_utf8(cmd.stdout.clone())?;
        let files = files.lines();

        files
            .into_iter()
            .map(parse_line)
            .map(|x| x)
            .collect::<Vec<_>>()
    };

    let mut actions = vec![];
    let read_content = |file_path: &str| -> FTResult<String> {
        std::fs::read_to_string(&file_path)
            .map_err(|e| crate::error::FTSyncError::ReadError(e).into())
    };

    for file in files.into_iter() {
        match file {
            ft_api::FileMode::Added(path) => {
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
            ft_api::FileMode::Renamed(p1, p2) => {
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
            ft_api::FileMode::Modified(path) => {
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
            ft_api::FileMode::Deleted(path) => {
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

    // let mut files: Vec<(String, String)> = vec![];
    // for (status, id, filename) in lines {
    //     println!("{:?}, {:?}, {:?}", status, id, filename);
    //     let content = fs::read_to_string(&filename)
    //         .map_err(| e | crate::error::FTSyncError::ReadError(e))?;
    //     files.push((id.to_string(), content));
    //    }

    println!("files {:#?}", actions);

    // ft_api::bulk_update::call(
    //     config.collection.as_str(),
    //     synced_hash.as_str(),
    //     latest_hash.as_str(),
    //     config.repo.as_str(),
    //     files,
    //     authcode.as_str(),
    // )?;

    Ok(())
}
