pub enum FileMode {
    Deleted(String),
    Renamed(String, String),
    Added(String),
    Modified(String),
}

fn parse_line(line: &str) -> self::FileMode {
    let sp = line.split('\t').collect::<Vec<_>>();
    let mode = sp[0].chars().next().unwrap();
    match mode {
        'A' => FileMode::Added(sp[1].to_string()),
        'M' => FileMode::Modified(sp[1].to_string()),
        'D' => FileMode::Deleted(sp[1].to_string()),
        'R' => FileMode::Renamed(sp[1].to_string(), sp[2].to_string()),
        _ => panic!("file with unknown mode : {}", line),
    }
}

pub fn ls_tree(hash: &str) -> crate::Result<Vec<FileMode>> {
    let cmd = std::process::Command::new("git")
        .args(&["ls-tree", "-r", "--name-only", hash.trim()])
        .output()?;
    let files = String::from_utf8(cmd.stdout)?;
    let files = files.lines();
    Ok(files
        .into_iter()
        .map(|x| FileMode::Added(x.to_string()))
        .collect())
}

pub fn diff(hash1: &str, hash2: &str) -> crate::Result<Vec<FileMode>> {
    let cmd = std::process::Command::new("git")
        .args(&["diff", "--name-status", hash1.trim(), hash2.trim()])
        .output()?;
    let files = String::from_utf8(cmd.stdout)?;
    let files = files.lines();

    Ok(files.into_iter().map(parse_line).collect::<Vec<_>>())
}

pub fn head() -> crate::types::Result<String> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn root_dir() -> crate::types::Result<String> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
