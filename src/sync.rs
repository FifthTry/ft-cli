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

            let content = vec![
                ftd::Section::Heading(ftd::Heading::new(0, config.collection.as_str())),
                ftd::Section::Markdown(ftd::Markdown::from_body(
                    &readme_content.unwrap_or_else(|| "".to_string()),
                )),
                ftd::Section::ToC(
                    tree.to_ftd_toc(config.root.as_str(), config.collection.as_str()),
                ),
            ];

            let collection =
                ftd::p1::to_string(&content.iter().map(|v| v.to_p1()).collect::<Vec<_>>());

            actions.push(ft_api::bulk_update::Action::Updated {
                id: config.collection.to_string(),
                content: format!("{}\n\n{}", collection, config.index_extra),
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
