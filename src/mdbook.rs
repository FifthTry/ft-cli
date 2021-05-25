pub fn handle_files(
    config: &crate::Config,
    files: &[crate::FileMode],
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let mdbook_config = mdbook::config::Config::default();

    let book = mdbook::MDBook::init(config.root.as_str())
        .create_gitignore(true)
        .with_config(mdbook_config)
        .build()
        .expect("Book generation failed");

    let mut actions = vec![];
    for file in files.iter() {
        actions.append(&mut self::handle(
            &file,
            config.root.as_str(),
            config.collection.as_str(),
        )?);
    }

    if files.iter().any(|v| {
        matches!(v, crate::FileMode::Created(_)) || matches!(v, crate::FileMode::Deleted(_))
    }) {
        // actions.push(self::index(&tree, config)?)
    }

    println!("actions: {:#?}", actions);
    // for x in book.book.iter() {
    //     println!("{:#?}", x);
    // }

    Ok(actions)
}

fn handle(
    file: &crate::FileMode,
    root: &str,
    collection: &str,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    if file.extension() != "md" {
        return Ok(vec![]);
    }
    let id = file.id_with_extension(root, collection);
    Ok(match file {
        crate::types::FileMode::Created(_) => {
            println!("Created: {}", id.as_str());
            vec![ft_api::bulk_update::Action::Added {
                content: file.raw_content(&id)?,
                id,
            }]
        }
        crate::types::FileMode::Modified(_) => {
            println!("Updated: {}", id.as_str());
            vec![ft_api::bulk_update::Action::Updated {
                content: file.raw_content(&id)?,
                id,
            }]
        }
        crate::types::FileMode::Deleted(_) => {
            vec![ft_api::bulk_update::Action::Deleted { id }]
        }
    })
}
