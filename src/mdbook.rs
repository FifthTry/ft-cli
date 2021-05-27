pub fn handle_files(
    config: &crate::Config,
    files: &[crate::FileMode],
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let (book_config, mdbook) = {
        let book_root = config.root.as_str();
        let book_root: std::path::PathBuf = book_root.into();
        let config_location = book_root.join("book.toml");

        // the book.json file is no longer used, so we should emit a warning to
        // let people know to migrate to book.toml
        if book_root.join("book.json").exists() {
            eprintln!("It appears you are still using book.json for configuration.");
            eprintln!("This format is no longer used, so you should migrate to the");
            eprintln!("book.toml format.");
            eprintln!("Check the user guide for migration information:");
            eprintln!("\thttps://rust-lang.github.io/mdBook/format/config.html");
        }

        let config = if config_location.exists() {
            mdbook::config::Config::from_disk(&config_location).expect("book.toml does not exists")
        } else {
            mdbook::config::Config::default()
        };
        (
            config.clone(),
            mdbook::MDBook::load_with_config(book_root, config).expect("Not able to load mdbook"),
        )
    };

    let src_dir = std::path::Path::new(config.root.as_str()).join(&book_config.book.src);
    let summary = self::summary_content(&src_dir).unwrap();

    let book = self::link_preprocess_book(
        &self::link_preprocessor_ctx(
            std::path::PathBuf::from(config.root.as_str()),
            book_config.clone(),
            "".to_string(),
        ),
        mdbook.book,
    );

    // println!("{:#?}", summary);
    // println!("{:#?}", book);

    let actions = {
        let mut actions = vec![];
        for file in files.iter() {
            actions.append(&mut self::handle(
                &summary,
                &book,
                &file,
                &src_dir.to_string_lossy(),
                config.collection.as_str(),
            )?);
        }

        // TODO: Need to remove it from this place
        actions.push(self::index(&summary, &book, config, &book_config.book.src)?);
        actions
    };

    Ok(actions)
}

fn handle(
    summary: &mdbook::book::Summary,
    book: &mdbook::book::Book,
    file: &crate::FileMode,
    root: &str,
    collection: &str,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    fn title(summary: &mdbook::book::Summary, file_path: &std::path::Path, id: &str) -> String {
        match file_path.file_name() {
            Some(p) => match self::chapter_title(summary, &p.to_string_lossy().to_string()) {
                Some(t) => t,
                None => id.to_string(),
            },
            None => id.to_string(),
        }
    }

    // TODO: Need to discuss with amitu
    if file.extension() != "md" {
        return Ok(vec![]);
    }

    // If the file is not part of SUMMARY.md then ignore the file
    let file_name = match file.path().file_name() {
        Some(name) => {
            if !is_summary_contains(summary, &name.to_string_lossy()) {
                return Ok(vec![]);
            }
            name.to_string_lossy().to_string()
        }
        None => return Ok(vec![]),
    };

    // TODO: If the file is SUMMARY.md or title-page.md modified, then return index
    // actions.push(self::index(&book.book, config)?);

    let id = match file.id(root, collection) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("{}", e.to_string());
            return Ok(vec![]);
        }
    };

    Ok(match file {
        crate::types::FileMode::Created(_) => {
            println!("Created: {}", id.as_str());
            let title = title(summary, &file.path(), id.as_str());
            vec![ft_api::bulk_update::Action::Added {
                content: file.raw_content_with_content(
                    &title,
                    &self::find_chapter_in_book(book, &file_name).expect("File content not found"),
                ), // file.raw_content(&title)?,
                id,
            }]
        }
        crate::types::FileMode::Modified(_) => {
            println!("Updated: {}", id.as_str());
            let title = title(summary, &file.path(), id.as_str());
            vec![ft_api::bulk_update::Action::Updated {
                content: file.raw_content_with_content(
                    &title,
                    &self::find_chapter_in_book(book, &file_name).expect("File content not found"),
                ), // file.raw_content(&title)?,
                id,
            }]
        }
        crate::types::FileMode::Deleted(_) => {
            vec![ft_api::bulk_update::Action::Deleted { id }]
        }
    })
}

fn index(
    summary: &mdbook::book::Summary,
    book: &mdbook::book::Book,
    config: &crate::Config,
    src: &std::path::Path,
) -> crate::Result<ft_api::bulk_update::Action> {
    let mut sections = vec![ftd::Section::Heading(ftd::Heading::new(
        0,
        &self::summary_title(summary).unwrap_or_else(|| config.collection.to_string()),
    ))];

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
        std::path::PathBuf::from(collection_id)
            .join(path)
            .with_extension("")
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
                mdbook::BookItem::Separator => {
                    // Separator
                    // PartTitle
                    // TODO: Need to discuss what to do with other sections
                }
                mdbook::BookItem::PartTitle(_) => {}
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

    // TODO: Will handle it later, this part of code is coming from `mdbook`
    // if create_missing {
    //     create_missing(&src_dir, &summary).with_context(|| "Unable to create missing chapters")?;
    // }

    Ok(summary)
}

fn chapter_title(summary: &mdbook::book::Summary, file_name: &str) -> Option<String> {
    self::get_by_name(summary, file_name).map(|x| x.name)
}

fn summary_title(summary: &mdbook::book::Summary) -> Option<String> {
    summary.title.clone()
}

// can be moved to separate mod `mdbook`
fn get_by_name(summary: &mdbook::book::Summary, file_name: &str) -> Option<mdbook::book::Link> {
    fn find_in_items(
        items: &[mdbook::book::SummaryItem],
        file_name: &str,
    ) -> Option<mdbook::book::Link> {
        for item in items {
            match match item {
                mdbook::book::SummaryItem::Link(link) => {
                    if let Some(name) = link.location.as_ref() {
                        if name.to_string_lossy().eq(file_name) {
                            return Some(link.clone());
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

fn is_summary_contains(summary: &mdbook::book::Summary, file_name: &str) -> bool {
    self::get_by_name(summary, file_name).is_some()
}

fn link_preprocessor_ctx(
    root: std::path::PathBuf,
    config: mdbook::config::Config,
    renderer: String,
) -> mdbook::preprocess::PreprocessorContext {
    mdbook::preprocess::PreprocessorContext::new(root, config, renderer)
}

fn link_preprocess_book(
    ctx: &mdbook::preprocess::PreprocessorContext,
    book: mdbook::book::Book,
) -> mdbook::book::Book {
    use mdbook::preprocess::Preprocessor;
    let link_preprocessor = mdbook::preprocess::LinkPreprocessor;
    match link_preprocessor.run(ctx, book) {
        Ok(book) => book,
        Err(e) => panic!("{}", e),
    }
}

fn find_chapter_in_book(book: &mdbook::book::Book, name: &str) -> Option<String> {
    fn util(book: &[mdbook::book::BookItem], name: &str) -> Option<String> {
        for book_item in book.iter() {
            match match book_item {
                mdbook::book::BookItem::Chapter(ch) => {
                    if let Some(path) = ch.path.as_ref() {
                        if path.eq(std::path::Path::new(name)) {
                            return Some(ch.content.to_string());
                        }
                    }
                    util(&ch.sub_items, name)
                }
                _ => None,
            } {
                Some(t) => return Some(t),
                None => continue,
            }
        }
        None
    }
    util(&book.sections, name)
}
