const RAW_EXTENSIONS: [&str; 4] = ["txt", "md", "mdx", "rst"];

pub fn handle_files(
    config: &crate::Config,
    files: &[crate::FileMode],
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let mut actions = vec![];
    let tree = crate::traverse::root_tree(&std::path::PathBuf::from(&config.root))?;

    for file in files.iter() {
        actions.append(&mut self::handle(
            &tree,
            &file,
            config.root.as_str(),
            config.collection.as_str(),
        )?);
    }

    if files.iter().any(|v| {
        matches!(v, crate::FileMode::Created(_)) || matches!(v, crate::FileMode::Deleted(_))
    }) {
        actions.push(self::index(&tree, config)?)
    }

    Ok(actions)
}

fn index(
    tree: &crate::traverse::Node,
    config: &crate::Config,
) -> crate::Result<ft_api::bulk_update::Action> {
    let readme_content = if let Some(readme) = tree.readme() {
        let file = crate::FileMode::Modified(readme);
        Some(file.content()?)
    } else {
        None
    };

    let mut content = vec![
        ftd::Section::Heading(ftd::Heading::new(
            0,
            config
                .title
                .clone()
                .unwrap_or_else(|| format!("`{}`", config.collection.as_str()))
                .as_str(),
        )),
        ftd::Section::Markdown(ftd::Markdown::from_body(
            &readme_content.unwrap_or_else(|| "".to_string()),
        )),
        ftd::Section::ToC(tree.to_ftd_toc(config.root.as_str(), config.collection.as_str())),
    ];

    content.extend_from_slice(&config.index_extra);

    println!("Updated: {}", config.collection.as_str());
    Ok(ft_api::bulk_update::Action::Updated {
        id: config.collection.to_string(),
        content: ftd::Document::new(&content).convert_to_string(),
    })
}

fn handle(
    tree: &crate::traverse::Node,
    file: &crate::FileMode,
    root: &str,
    collection: &str,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    if !RAW_EXTENSIONS.contains(&file.extension().as_str()) {
        return Ok(vec![]);
    }

    let id = file.id_with_extension(root, collection);

    Ok(match file {
        crate::types::FileMode::Created(ref file_path) => {
            println!("Created: {}", id.as_str());
            let mut actions = ancestors(tree, file_path, root, collection);
            actions.push(ft_api::bulk_update::Action::Added {
                content: file.raw_content(&format!("`{}`", id))?,
                id,
            });
            actions
        }

        crate::types::FileMode::Modified(_) => {
            println!("Updated: {}", id.as_str());
            vec![ft_api::bulk_update::Action::Updated {
                content: file.raw_content(&format!("`{}`", id))?,
                id,
            }]
        }

        crate::types::FileMode::Deleted(ref file_path) => {
            println!("Deleted: {}", id.as_str());
            let mut actions = ancestors(tree, file_path, root, collection);
            actions.push(ft_api::bulk_update::Action::Deleted { id });
            actions
        }
    })
}

fn ancestors(
    root_tree: &crate::traverse::Node,
    file_path: &str,
    root_dir: &str,
    collection: &str,
) -> Vec<ft_api::bulk_update::Action> {
    crate::traverse::ancestors(root_tree, file_path)
        .iter()
        .filter(|x| !x.readme_exists())
        .map(|node| ft_api::bulk_update::Action::Updated {
            id: node
                .document_id(root_dir, collection)
                .to_string_lossy()
                .to_string(),
            content: node.to_markdown(root_dir, collection),
        })
        .collect()
}
