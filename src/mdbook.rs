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

    println!("{:#?}", book.book);

    Ok(vec![])
}
