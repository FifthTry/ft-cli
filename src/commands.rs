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

    let (synced_hash, updated_on) = crate::fifthtry::status::call(authcode.as_str())?;

    println!("Config: {}", config_file_path);
    println!("Backend: {}", config.backend.to_string());
    println!("Root: {}", config.root);
    println!("Last Synced Hash: {}", if synced_hash.is_empty() {"Never Synced"} else {synced_hash.as_str()});
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
    use std::fs;
    use std::process::Command;

    let authcode = match &config.auth {
        Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let (synced_hash, _) = crate::fifthtry::status::call(authcode.as_str())?;

    let output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;
    let latest_hash = String::from_utf8(output.stdout)?;

    let root_dir_output = Command::new("git").arg("rev-parse").arg("--show-toplevel").output()?;
    let root_dir = String::from_utf8(root_dir_output.stdout)?;

    let data_dir = std::path::Path::new(&root_dir.trim()).join(&config.root);

    let data_dir = match data_dir.to_str() {
        Some(s) => s.to_string() + "/",
        None => "/".to_string()
    };
    println!("{:?}", data_dir);



    let git_diff = if synced_hash.is_empty() {
        Command::new("git")
            .args(&["ls-tree", "-r", "--name-only", latest_hash.trim()])
            .output()?
    } else {
        Command::new("git")
            .args(&["diff", "--name-only", synced_hash.trim(), latest_hash.trim()])
            .output()?
    };

    let lines = String::from_utf8(git_diff.stdout)?;

    let lines = lines.lines();
    let mut files: Vec<(String, String)> = vec![];

    let lines: Vec<_> = lines.into_iter()
        .filter(|x| config.backend.accept(std::path::Path::new(x)))
        .map(|x| std::path::Path::new(&root_dir.trim()).join(x))
        .map(|x| x.to_str().map(|x| x.to_string()))
        .filter_map(|x| x)
        .map(|x| (x.replacen(&data_dir, "", 1), x))
        .map(|(x, y) | (x.replacen(".ftd","", 1), y))
        .collect();

    dbg!(&lines);

    for (id, filename) in lines {
        let content = fs::read_to_string(&filename)
            .map_err(| e | crate::error::FTSyncError::ReadError(e))?;
        files.push((id.to_string(), content));
    }

    println!("files {:?}", files);

    crate::fifthtry::bulk_update::call(
        config.collection.as_str(),
        synced_hash.as_str(),
        latest_hash.as_str(),
        config.repo.as_str(),
        files,
        authcode.as_str(),
    )?;

    Ok(())
}
