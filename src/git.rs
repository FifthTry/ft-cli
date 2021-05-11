pub enum FileMode {
    Deleted(String),
    Added(String),
    Modified(String),
}

pub fn ls_tree(
    hash: &str,
    git_root: &str,
    root_dir: &str,
    pattern: &Option<String>,
) -> crate::Result<Vec<FileMode>> {
    let args = match pattern {
        Some(p) => vec!["ls-tree", "-r", "--name-only", hash.trim(), p.as_str()],
        None => vec!["ls-tree", "-r", "--name-only", hash.trim()],
    };

    let cmd = std::process::Command::new("git").args(&args).output()?;
    let files = String::from_utf8(cmd.stdout)?;
    let files = files.lines();
    Ok(files
        .into_iter()
        .filter_map(|x| {
            let path = git_root.to_string() + "/" + x;
            if path.starts_with(root_dir) {
                Some(FileMode::Added(git_root.to_string() + "/" + x))
            } else {
                None
            }
        })
        .collect())
}

pub fn diff(
    hash1: &str,
    hash2: &str,
    git_root: &str,
    root_dir: &str,
    pattern: &Option<String>,
) -> crate::Result<Vec<FileMode>> {
    let args = match pattern {
        Some(p) => vec![
            "diff",
            "--name-status",
            "--no-renames",
            hash1.trim(),
            hash2.trim(),
            p.as_str(),
        ],
        None => vec![
            "diff",
            "--name-status",
            "--no-renames",
            hash1.trim(),
            hash2.trim(),
        ],
    };

    let cmd = std::process::Command::new("git").args(&args).output()?;
    let files = String::from_utf8(cmd.stdout)?;
    let files = files.lines();

    Ok(files
        .into_iter()
        .filter_map(|line: &str| {
            let sp = line.split('\t').collect::<Vec<_>>();
            let mode = sp[0].chars().next().unwrap();
            let path = git_root.to_string() + "/" + sp[1];
            if path.starts_with(root_dir) {
                Some(match mode {
                    'A' => FileMode::Added(path),
                    'M' => FileMode::Modified(path),
                    'D' => FileMode::Deleted(path),
                    _ => panic!("file with unknown mode : {}", line),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
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
