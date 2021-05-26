pub fn handle_files(
    config: &crate::Config,
    files: &[crate::FileMode],
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let (book_config, book) = {
        let book_root = config.root.as_str();
        let book_root: std::path::PathBuf = book_root.into();
        let config_location = book_root.join("book.toml");
        let config = if config_location.exists() {
            mdbook::config::Config::from_disk(&config_location).expect("")
        } else {
            mdbook::config::Config::default()
        };
        (
            config.clone(),
            mdbook::MDBook::load_with_config(book_root, config).expect(""),
        )
    };
    // println!("{:#?}", book.book);

    let root_path = std::path::Path::new(config.root.as_str()).join(&book_config.book.src);
    let mut actions = vec![];
    for file in files.iter() {
        actions.append(&mut self::handle(
            &file,
            &root_path.to_string_lossy(),
            config.collection.as_str(),
        )?);
    }

    // TODO: Need to remove it from this place
    actions.push(self::index(&book.book, config, &book_config.book.src)?);

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

    // TODO: If the file is not part of SUMMARY.md then ignore the file
    //
    // TODO: If the file is SUMMARY.md or title-page.md modified, then return index
    // actions.push(self::index(&book.book, config)?);

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
    src: &std::path::Path,
) -> crate::Result<ft_api::bulk_update::Action> {
    let mut sections = vec![];

    let title_page = std::path::Path::new(&config.root)
        .join(src)
        .join("title-page.md");
    if title_page.exists() {
        let content = std::fs::read_to_string(&title_page)
            .map_err(|e| crate::Error::ReadError(e, title_page.to_string_lossy().to_string()))?;
        sections.push(ftd::Section::Markdown(ftd::Markdown::from_body(
            content.as_str(),
        )));
    }

    sections.push(ftd::Section::ToC(self::to_ftd_toc(
        book,
        config.collection.as_str(),
    )));
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
                    let id = path_to_doc_id(
                        &match chapter.path.as_ref() {
                            Some(p) => p.to_string_lossy().to_string(),
                            None => "virtual-document".to_string(),
                        },
                        collection_id,
                    )
                    .to_string_lossy()
                    .to_string();

                    let title = format!(
                        "{}{}",
                        chapter
                            .number
                            .as_ref()
                            .map(|x| format!("`{}` ", x))
                            .unwrap_or_else(|| "".to_string()),
                        chapter.name
                    );

                    let mut item = ftd::toc::TocItem::with_title_and_id(&title, &id);
                    item.children = to_ftd_items(&chapter.sub_items, collection_id);
                    toc_items.push(item);
                }
                _ => {
                    // Separator
                    // PartTitle
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
