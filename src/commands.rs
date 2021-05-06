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

    let (synced_hash, _) = ft_api::status::call(authcode.as_str())?;

    let output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;
    let latest_hash = String::from_utf8(output.stdout)?;

    let root_dir_output = Command::new("git").arg("rev-parse").arg("--show-toplevel").output()?;
    let root_dir = String::from_utf8(root_dir_output.stdout)?;

    let data_dir = std::path::Path::new(&root_dir.trim()).join(&config.root);

    let data_dir = match data_dir.to_str() {
        Some(s) => s.to_string() + "/",
        None => "/".to_string()
    };
    // println!("{:?}", data_dir);

    fn parse_line(line: &str) -> (String, String) {
        let sp = line.split("\t").collect::<Vec<_>>();
        let mode = sp[0].chars().next().unwrap();

        let file_name = match mode {
            'A' | 'M' | 'D' => sp[1],
            'R' => sp[2],
            _ => panic!("file with unknown mode : {}", line)
        };
        (mode.to_string(), file_name.to_string())
    }

    let files = if synced_hash.is_empty() {
        let cmd = Command::new("git")
            .args(&["ls-tree", "-r", "--name-only", latest_hash.trim()])
            .output()?;
        let files = String::from_utf8(cmd.stdout.clone())?;
        let files = files.lines();
        files.into_iter().map(|x| ("A".to_string(), x.to_string())).collect()
    } else {
        let cmd = Command::new("git")
            .args(&["diff", "--name-status", synced_hash.trim(), latest_hash.trim()])
            .output()?;
        let files = String::from_utf8(cmd.stdout.clone())?;
        let files = files.lines();

        files.into_iter()
            .map(parse_line)
            .map(|x| x)
            .collect::<Vec<_>>()
    };

    let lines: Vec<_> = files.into_iter()
        .filter(|x| config.backend.accept(std::path::Path::new(&x.1)))
        .map(|x| (x.0, std::path::Path::new(&root_dir.trim()).join(x.1)))
        .map(|x|
            if let Some(p) =  x.1.to_str().map(|x| x.to_string()) {
                Some ((x.0, p))
            } else { None })
        .filter_map(|x| x)
        .map(|x| (x.0, x.1.replacen(&data_dir, "", 1), x.1))
        .map(|(x, y, z) | (x, y.replacen(".ftd","", 1), z))
        .collect();

    let mut files: Vec<(String, String)> = vec![];

    for (status, id, filename) in lines {
        println!("{:?}, {:?}, {:?}", status, id, filename);
        let content = fs::read_to_string(&filename)
            .map_err(| e | crate::error::FTSyncError::ReadError(e))?;
        files.push((id.to_string(), content));
    }

    // println!("files {:?}", files);

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
