pub enum FileMode {
    Deleted(String),
    Renamed(String, String),
    Added(String),
    Modified(String),
}

fn parse_line(line: &str) -> self::FileMode {
    let sp = line.split("\t").collect::<Vec<_>>();
    let mode = sp[0].chars().next().unwrap();
    match mode {
        'A' => crate::git::FileMode::Added(sp[1].to_string()),
        'M' => crate::git::FileMode::Modified(sp[1].to_string()),
        'D' => crate::git::FileMode::Deleted(sp[1].to_string()),
        'R' => crate::git::FileMode::Renamed(sp[1].to_string(), sp[2].to_string()),
        _ => panic!("file with unknown mode : {}", line),
    }
}

pub fn git_ls_tree(hash: &str) -> crate::types::Result<Vec<FileMode>> {
    let cmd = std::process::Command::new("git")
        .args(&["ls-tree", "-r", "--name-only", hash.trim()])
        .output()?;
    let files = String::from_utf8(cmd.stdout.clone())?;
    let files = files.lines();
    Ok(files
        .into_iter()
        .map(|x| self::FileMode::Added(x.to_string()))
        .collect())
}

pub fn git_diff(hash1: &str, hash2: &str) -> crate::types::Result<Vec<FileMode>> {
    let cmd = std::process::Command::new("git")
        .args(&["diff", "--name-status", hash1.trim(), hash2.trim()])
        .output()?;
    let files = String::from_utf8(cmd.stdout.clone())?;
    let files = files.lines();

    Ok(files
        .into_iter()
        .map(parse_line)
        .map(|x| x)
        .collect::<Vec<_>>())
}
