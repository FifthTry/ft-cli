pub fn ls_tree(hash: &str, git_root: &str, root_dir: &str) -> crate::Result<Vec<crate::FileMode>> {
    let files: String = if crate::is_test() {
        realm_client::mock(
            Some("ls_tree".to_string()),
            // serde_json::json! ({"hash": hash, "git_root": git_root, "root_dir": root_dir}),
            serde_json::json!({ "hash": hash }),
        )
    } else {
        let cmd = std::process::Command::new("git")
            .args(&["ls-tree", "-r", "--name-only", hash.trim()])
            .output()?;

        String::from_utf8(cmd.stdout)?
    };

    let files = files.lines();
    Ok(files
        .into_iter()
        .filter_map(|x| {
            let path = git_root.to_string() + "/" + x;
            if path.starts_with(root_dir) {
                Some(crate::FileMode::Created(git_root.to_string() + "/" + x))
            } else {
                None
            }
        })
        .collect())
}

pub fn changed_files(
    hash1: &str,
    hash2: &str,
    git_root: &str,
    root_dir: &str,
) -> crate::Result<Vec<crate::FileMode>> {
    let files: String = if crate::is_test() {
        realm_client::mock(
            Some("changed_files".to_string()),
            // serde_json::json! ({"hash1": hash1, "hash2": hash2, "git_root": git_root, "root_dir": root_dir}),
            serde_json::json! ({"hash1": hash1, "hash2": hash2}),
        )
    } else {
        let cmd = std::process::Command::new("git")
            .args(&[
                "diff",
                "--name-status",
                "--no-renames",
                hash1.trim(),
                hash2.trim(),
            ])
            .output()?;
        String::from_utf8(cmd.stdout)?
    };

    let files = files.lines();

    Ok(files
        .into_iter()
        .filter_map(|line: &str| {
            let sp = line.split('\t').collect::<Vec<_>>();
            let mode = sp[0].chars().next().unwrap();
            let path = git_root.to_string() + "/" + sp[1];
            if path.starts_with(root_dir) {
                Some(match mode {
                    'A' => crate::FileMode::Created(path),
                    'M' => crate::FileMode::Modified(path),
                    'D' => crate::FileMode::Deleted(path),
                    _ => panic!("file with unknown mode : {}", line),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}

pub fn head() -> crate::Result<String> {
    if crate::is_test() {
        return Ok(realm_client::mock(
            Some("head".to_string()),
            serde_json::json!({}),
        ));
    }

    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn root_dir() -> crate::Result<String> {
    if crate::is_test() {
        return Ok(std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string());
    }

    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
