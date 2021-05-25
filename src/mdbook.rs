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

    // TODO: Need to discuss it with amitu about appending "src"
    let root_path = std::path::Path::new(config.root.as_str()).join("src");
    let mut actions = vec![];
    for file in files.iter() {
        actions.append(&mut self::handle(
            &file,
            &root_path.to_string_lossy(),
            config.collection.as_str(),
        )?);
    }

    self::index(&book.book, config)?;
    if files.iter().any(|v| {
        matches!(v, crate::FileMode::Created(_)) || matches!(v, crate::FileMode::Deleted(_))
    }) {
        actions.push(self::index(&book.book, config)?)
    }

    println!("actions: {:#?}", actions);
    Ok(actions)
}

fn handle(
    file: &crate::FileMode,
    root: &str,
    collection: &str,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    // TODO: Need to discuss with amitu
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

fn index(
    book: &mdbook::book::Book,
    config: &crate::Config,
) -> crate::Result<ft_api::bulk_update::Action> {
    let mut sections = vec![ftd::Section::ToC(self::to_ftd_toc(
        book,
        config.collection.as_str(),
    ))];
    sections.extend_from_slice(&config.index_extra);

    println!("Updated: {}", config.collection.as_str());
    Ok(ft_api::bulk_update::Action::Updated {
        id: config.collection.to_string(),
        content: ftd::Document::new(&sections).convert_to_string(),
    })
}

fn to_ftd_toc(book: &mdbook::book::Book, collection_id: &str) -> ftd::toc::ToC {
    fn path_to_doc_id(path: &str, collection_id: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(collection_id).join(path)
    }

    fn to_ftd_items(items: &[mdbook::BookItem], collection_id: &str) -> Vec<ftd::toc::TocItem> {
        let mut toc_items = vec![];
        for item in items.iter() {
            match item {
                mdbook::BookItem::Chapter(chapter) => {
                    // TODO: chapter.source_path, chapter.path both are optional
                    let id = path_to_doc_id(
                        &chapter.path.as_ref().unwrap().to_string_lossy(),
                        collection_id,
                    )
                    .to_string_lossy()
                    .to_string();
                    // println!(
                    //     "Title: {}, Id: {:?}, {:?}",
                    //     chapter, chapter.source_path, &id
                    // );
                    let mut item = ftd::toc::TocItem::with_title_and_id(&chapter.to_string(), &id);
                    item.children = to_ftd_items(&chapter.sub_items, collection_id);
                    toc_items.push(item);
                }
                _ => {
                    // TODO: Need to discuss what to do with other sections
                }
            }
        }
        toc_items
    }

    ftd::ToC {
        items: to_ftd_items(&book.sections, collection_id),
    }
}
