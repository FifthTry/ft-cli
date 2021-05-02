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

    let synced_hash = crate::fifthtry::status::call(authcode.as_str())?;

    println!("Config: {}", config_file_path);
    println!("Backend: {}", config.backend.to_string());
    println!("Root: {}", config.root);
    println!("Last Synced Hash: {}", if synced_hash.is_empty() {"Never Synced"} else {synced_hash.as_str()});
    //println!("Last Sync On: {}", "");

    Ok(())
}

pub fn sync(config: crate::config::Config, _dry_run: bool) -> FTResult<()> {
    use crate::types::Auth;
    use std::fs;
    use std::process::Command;

    let authcode = match &config.auth {
        Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let synced_hash = crate::fifthtry::status::call(authcode.as_str())?;

    let output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;
    let latest_hash = String::from_utf8(output.stdout)?;

    let git_diff = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(&latest_hash)
        .arg(&synced_hash)
        .output()?;

    let lines = String::from_utf8(git_diff.stdout)?;

    let lines = lines.lines();

    let mut files: Vec<(String, String)> = vec![];
    let lines: Vec<_> = lines.into_iter()
        .filter(|x| config.backend.accept(std::path::Path::new(x)))
        .collect();

    for filename in lines {
        let content = fs::read_to_string(filename)
            .map_err(| e | crate::error::FTSyncError::ReadError(e))?;
        let doc_id = filename.to_string();
        files.push((doc_id, content));
    }

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
