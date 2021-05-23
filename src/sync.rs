pub fn sync(config: &crate::Config) -> crate::Result<()> {
    let auth_code = match &config.auth {
        crate::Auth::AuthCode(s) => s.to_string(),
        _ => return Ok(()),
    };

    let latest_hash = crate::git::head()?;

    let status = ft_api::sync_status(
        config.collection.as_str(),
        auth_code.as_str(),
        &crate::utils::platform()?,
        &crate::utils::client_version(),
    )?;

    let actions = {
        let mut actions = vec![];

        let tree = crate::traverse::root_tree(&std::path::PathBuf::from(&config.root))?;

        let files = if status.last_synced_hash.is_empty() {
            crate::git::ls_tree(&latest_hash, config.root.as_str())?
        } else {
            crate::git::changed_files(&status.last_synced_hash, &latest_hash, config.root.as_str())?
        };

        for file in files.into_iter() {
            actions.append(&mut match config.backend {
                crate::Backend::FTD => {
                    crate::ftd::handle(file, config.root.as_str(), config.collection.as_str())?
                }
                crate::Backend::Raw => crate::raw::handle(
                    &tree,
                    file,
                    config.root.as_str(),
                    config.collection.as_str(),
                )?,
            });
        }

        if config.backend.is_raw() {
            let readme_content = if let Some(readme) = tree.readme() {
                let file = crate::FileMode::Modified(readme);
                Some(file.content()?)
            } else {
                None
            };

            let collection_toc =
                tree.collection_toc(config.root.as_str(), config.collection.as_str());

            actions.push(ft_api::bulk_update::Action::Updated {
                id: config.collection.to_string(),
                content: format!(
                    "-- h1: {}\n\n\n{}\n{}\n\n\n{}",
                    config.collection,
                    config.index_extra,
                    match readme_content {
                        Some(c) => format!("\n-- markdown:\n\n{}", c),
                        None => {
                            "".to_string()
                        }
                    },
                    collection_toc
                ),
            })
        }
        actions
    };

    // println!("{:#?}", actions);

    let st = std::time::Instant::now();

    ft_api::bulk_update(
        config.collection.as_str(),
        status.last_synced_hash.as_str(),
        latest_hash.as_str(),
        config.repo.as_str(),
        actions,
        auth_code.as_str(),
        crate::utils::platform()?,
        crate::utils::client_version(),
    )?;

    if crate::is_test() {
        println!("Synced successfully.");
    } else {
        println!("Synced successfully: {}.", crate::utils::elapsed(st));
    }

    Ok(())
}
