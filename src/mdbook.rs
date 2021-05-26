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

    let src_dir = std::path::Path::new(config.root.as_str()).join(&book_config.book.src);

    let summary = self::summary_content(&src_dir).unwrap();

    // println!("{:#?}", summary);
    // println!("{:#?}", book.book);

    let mut actions = vec![];
    for file in files.iter() {
        actions.append(&mut self::handle(
            &summary,
            &file,
            &src_dir.to_string_lossy(),
            config.collection.as_str(),
        )?);
    }

    // TODO: Need to remove it from this place
    actions.push(self::index(&book.book, config, &book_config.book.src)?);

    println!("actions: {:#?}", actions);
    Ok(actions)
}

fn handle(
    summary: &mdbook::book::Summary,
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
        crate::types::FileMode::Created(file_path) => {
            println!("Created: {}", id.as_str());
            let title = self::chapter_title(&summary, &std::path::Path::new(file_path))
                .unwrap_or_else(|| id.clone());
            vec![ft_api::bulk_update::Action::Added {
                content: file.raw_content(&title)?,
                id,
            }]
        }
        crate::types::FileMode::Modified(file_path) => {
            println!("Updated: {}", id.as_str());
            let title = self::chapter_title(&summary, &std::path::Path::new(file_path))
                .unwrap_or_else(|| id.clone());
            vec![ft_api::bulk_update::Action::Updated {
                content: file.raw_content(&title)?,
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

pub fn summary_content(src_dir: &std::path::Path) -> mdbook::errors::Result<mdbook::book::Summary> {
    use std::io::Read;
    let summary_md = src_dir.join("SUMMARY.md");

    let mut summary_content = String::new();
    std::fs::File::open(&summary_md)?.read_to_string(&mut summary_content)?;

    let summary = mdbook::book::parse_summary(&summary_content)?;

    // if create_missing {
    //     create_missing(&src_dir, &summary).with_context(|| "Unable to create missing chapters")?;
    // }

    Ok(summary)
}

fn chapter_title(summary: &mdbook::book::Summary, file_name: &std::path::Path) -> Option<String> {
    fn find_in_items(
        items: &[mdbook::book::SummaryItem],
        file_name: &std::path::Path,
    ) -> Option<String> {
        for item in items {
            match match item {
                mdbook::book::SummaryItem::Link(link) => {
                    if let Some(name) = link.location.as_ref() {
                        if name.eq(file_name) {
                            return Some(link.name.to_string());
                        }
                    }
                    find_in_items(&link.nested_items, file_name)
                }
                mdbook::book::SummaryItem::PartTitle(_) => None,
                mdbook::book::SummaryItem::Separator => None,
            } {
                Some(s) => return Some(s),
                None => continue,
            };
        }
        None
    }

    find_in_items(&summary.numbered_chapters, file_name)
        .or_else(|| find_in_items(&summary.prefix_chapters, file_name))
        .or_else(|| find_in_items(&summary.suffix_chapters, file_name))
}

fn summary_title(summary: &mdbook::book::Summary) -> Option<String> {
    summary.title.clone()
}
