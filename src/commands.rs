use crate::types::FTResult;
use std::process::Command;

fn status(config: crate::types::Config, config_file_path: String) -> FTResult<()> {
    /*
    Config: ../.ft-sync.p1
    Backend: mdBook
    Root: docs
    Last Sync On: 2021-04-21 3:05PM (CST).
    */

    let synced_hash = crate::fifthtry::status::call(authcode.as_str())?;

    println!("Config: {}", config_file_path);
    println!("Backend: {}", config.backend.to_string());
    println!("Root: {}", config.root);
    println!("Last Synced Hash: {}", if synced_hash.is_empty() "Never Synced" else synced_hash);
    //println!("Last Sync On: {}", "");

    OK(())
}

fn sync(config: crate::types::Config, _dry_run: bool) -> FTResult<()> {
    use crate::types::Auth;
    use std::fs;
    use std::process::Command;

    let authcode = match config.auth {
        Auth::AuthCode(s) => s,
        _ => return Ok(()),
    };

    let synced_hash = crate::fifthtry::status::call(authcode.as_str())?;

    let output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;
    let latest_hash = String::from_utf8(output.stdout)?;

    let git_diff = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(latest_hash)
        .arg(synced_hash)
        .output()?;

    let lines = String::from_utf8(git_diff.stdout)?.lines();

    let mut files: Vec<(String, String)> = vec![];
    for filename in lines {
        let content = fs::read_to_string(filename).unwrap();
        files.push((filename.to_string(), content));
    }

    crate::fifthtry::bulk_update::call(
        config.collection.as_str(),
        synced_hash.as_str(),
        latest_hash.as_str(),
        config.repo.as_str(),
        files,
        auth_code,
    );

    OK(())
}
