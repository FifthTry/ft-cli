fn read_content(file_path: &str) -> crate::Result<String> {
    std::fs::read_to_string(&file_path)
        .map_err(|e| crate::Error::ReadError(e, file_path.to_string()).into())
}

fn to_docid(path: &str, collection: &str, root_dir: &str) -> String {
    let t = std::path::Path::new(&path)
        .strip_prefix(root_dir)
        .unwrap()
        .with_extension("")
        .to_str()
        .unwrap()
        .to_string();
    if t == "index" {
        collection.to_string()
    } else {
        collection.to_string() + "/" + t.as_str()
    }
}

fn to_docid_with_extension(path: &str, collection: &str, root_dir: &str) -> String {
    let t = std::path::Path::new(&path)
        .strip_prefix(root_dir)
        .unwrap()
        .to_string_lossy()
        .to_string();

    if t == "index" {
        collection.to_string()
    } else {
        collection.to_string() + "/" + t.as_str()
    }
}

pub fn read_ftd_files(
    config: &crate::Config,
    root_dir: &str,
    files: Vec<crate::git::FileMode>,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let mut actions = vec![];

    for file in files.into_iter() {
        match file {
            crate::git::FileMode::Added(path) => {
                if config.backend.accept(std::path::Path::new(&path)) {
                    let docid = self::to_docid(&path, &config.collection, &root_dir);
                    println!("Added new: {}", path);
                    actions.push(ft_api::bulk_update::Action::Added {
                        id: docid,
                        content: self::read_content(&path)?,
                    });
                }
            }

            crate::git::FileMode::Modified(path) => {
                if config.backend.accept(std::path::Path::new(&path)) {
                    let docid = self::to_docid(&path, &config.collection, &root_dir);
                    println!("Updated: {}", path);
                    actions.push(ft_api::bulk_update::Action::Updated {
                        id: docid,
                        content: self::read_content(&path)?,
                    });
                }
            }
            crate::git::FileMode::Deleted(path) => {
                if config.backend.accept(std::path::Path::new(&path)) {
                    let docid = self::to_docid(&path, &config.collection, &root_dir);
                    println!("Deleted: {}", path);
                    actions.push(ft_api::bulk_update::Action::Deleted { id: docid });
                }
            }
        }
    }
    Ok(actions)
}

fn read_raw_files(
    config: &crate::Config,
    root_dir: &str,
    files: Vec<crate::git::FileMode>,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let mut actions = vec![];

    let to_raw = |title: &str, content: &str| {
        let content = content
            .lines()
            .map(|x| {
                if x.starts_with("--") || x.starts_with("---") {
                    "\\".to_string() + x
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let t = format!("-- h0: {}\n\n-- raw: \n\n {}", title, content);
        // println!("{}", t);
        t
    };

    for file in files {
        match file {
            crate::git::FileMode::Added(path) => {
                let docid = self::to_docid_with_extension(&path, &config.collection, &root_dir);
                println!("Added new: {}", path);
                let content = self::read_content(&path)?;
                actions.push(ft_api::bulk_update::Action::Added {
                    content: to_raw(&docid, &content),
                    id: docid,
                });
            }
            crate::git::FileMode::Modified(path) => {
                let docid = self::to_docid_with_extension(&path, &config.collection, &root_dir);
                println!("Added new: {}", path);
                let content = self::read_content(&path)?;
                actions.push(ft_api::bulk_update::Action::Updated {
                    content: to_raw(&docid, &content),
                    id: docid,
                });
            }
            crate::git::FileMode::Deleted(path) => {
                if config.backend.accept(std::path::Path::new(&path)) {
                    let docid = self::to_docid_with_extension(&path, &config.collection, &root_dir);
                    println!("Deleted: {}", path);
                    actions.push(ft_api::bulk_update::Action::Deleted { id: docid });
                }
            }
        }
    }
    Ok(actions)
}

pub fn sync(config: &crate::Config, _dry_run: bool) -> crate::Result<()> {
    let auth_code = match &config.auth {
        crate::Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let latest_hash = crate::git::head()?;
    let git_root = crate::git::root_dir()?;

    let root_dir = {
        let root_dir = config.root_abs_path();
        if !root_dir.starts_with(&git_root) {
            panic!(
                "The root directory: {:?} is not inside git dir: {}",
                root_dir.as_os_str(),
                &git_root
            )
        }

        root_dir.to_string_lossy().to_string()
    };

    let status = ft_api::sync_status(
        config.collection.as_str(),
        auth_code.as_str(),
        &crate::utils::platform()?,
        &crate::utils::client_version(),
    )?;

    let files = if status.last_synced_hash.is_empty() {
        crate::git::ls_tree(&latest_hash, &git_root, &root_dir)?
    } else {
        crate::git::changed_files(&status.last_synced_hash, &latest_hash, &git_root, &root_dir)?
    };

    let t = self::root_tree(&std::path::PathBuf::from(&root_dir))?;
    println!("{:#?}", &t);
    let toc = self::tree_to_toc_util(&t);
    println!("{}", toc);

    let _actions = {
        match config.backend {
            crate::Backend::FTD => self::read_ftd_files(&config, root_dir.as_str(), files)?,
            crate::Backend::RAW => self::read_raw_files(&config, root_dir.as_str(), files)?,
            _ => panic!(),
        }
    };

    let st = std::time::Instant::now();

    // ft_api::bulk_update(
    //     config.collection.as_str(),
    //     status.last_synced_hash.as_str(),
    //     latest_hash.as_str(),
    //     config.repo.as_str(),
    //     actions,
    //     auth_code.as_str(),
    //     crate::utils::platform()?,
    //     crate::utils::client_version(),
    // )?;

    println!("Synced successfully: {}", crate::utils::elapsed(st));

    Ok(())
}

// Can be simplified to
/*
#[derive(Debug)]
struct Node {
    Dir{ path: String, children: Vec<Node> }
    File{ path: String }
}
 */

#[derive(Debug)]
struct Node {
    pub is_dir: bool,
    pub path: String,
    pub children: Vec<Node>,
}

fn root_tree(root_dir: &std::path::Path) -> crate::Result<Node> {
    let root = Node {
        is_dir: true,
        path: root_dir.to_string_lossy().to_string(),
        children: traverse_tree(root_dir)?,
    };
    Ok(root)
}

fn traverse_tree(root_dir: &std::path::Path) -> crate::Result<Vec<Node>> {
    let mut children = vec![];

    for entry in std::fs::read_dir(root_dir)? {
        let p = entry?.path();
        if p.is_dir() {
            children.push(Node {
                is_dir: true,
                path: p.to_string_lossy().to_string(),
                children: traverse_tree(&p)?,
            });
        } else {
            children.push(Node {
                is_dir: false,
                path: p.to_string_lossy().to_string(),
                children: vec![],
            });
        }
    }
    Ok(children)
}

fn tree_to_toc_util(node: &Node) -> String {
    let mut toc = String::new();
    tree_to_toc(node, 0, &mut toc);
    toc
}

// Incomplete, Need to do small change
fn tree_to_toc(node: &Node, level: usize, toc_string: &mut String) {
    // toc_string.push_str(&format!(
    //     "{: >width$}- {path}\n",
    //     "",
    //     width = level,
    //     path = &node.path
    // ));
    for x in node.children.iter() {
        toc_string.push_str(&format!(
            "{: >width$}- {path}\n",
            "",
            width = level + 2,
            path = &x.path
        ));
        if x.is_dir {
            tree_to_toc(&x, level + 2, toc_string);
        }
    }
}
