pub fn handle_files(
    config: &crate::Config,
    files: &[crate::FileMode],
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let mut actions = vec![];

    for file in files.iter() {
        actions.append(&mut self::handle(
            file,
            config.root.as_str(),
            config.collection.as_str(),
            config.preserve_meta,
        )?);
    }
    Ok(actions)
}

fn handle(
    file: &crate::FileMode,
    root_dir: &str,
    collection: &str,
    preserve_meta: bool,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    if file.extension() != "ftd" {
        return Ok(vec![]);
    }

    let id = match file.id(root_dir, collection) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("{}", e.to_string());
            return Ok(vec![]);
        }
    };

    Ok(match file {
        crate::types::FileMode::Created(_) => {
            println!("Created: {}", id.as_str());
            vec![ft_api::bulk_update::Action::Added {
                id,
                preserve_meta,
                content: file.content()?,
            }]
        }

        crate::types::FileMode::Modified(_) => {
            println!("Updated: {}", id.as_str());
            vec![ft_api::bulk_update::Action::Updated {
                id,
                preserve_meta,
                content: file.content()?,
            }]
        }

        crate::types::FileMode::Deleted(_) => {
            println!("Deleted: {}", id.as_str());
            vec![ft_api::bulk_update::Action::Deleted { id }]
        }
    })
}
