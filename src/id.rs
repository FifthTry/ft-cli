pub fn to_document_id(path: &str, root_dir: &str, collection_id: &str) -> std::path::PathBuf {
    let path_without_root = std::path::Path::new(path)
        .strip_prefix(root_dir)
        .unwrap_or_else(|_| panic!("path `{}` is not starts with root_dir `{}`", path, root_dir));
    std::path::PathBuf::from(collection_id).join(&path_without_root)
}
