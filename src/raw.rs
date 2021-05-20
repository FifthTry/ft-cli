pub fn handle(
    file: crate::FileMode,
    root_dir: &str,
    collection: &str,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let id = file.id_with_extension(root_dir, collection);

    Ok(match file {
        crate::types::FileMode::Added(ref path) => {
            println!("Added new: {}", path);
            // TODO: find all parent directories and append them here
            // let all = all_parents();
            vec![ft_api::bulk_update::Action::Added {
                id,
                content: file.raw_content()?,
            }]
        }

        crate::types::FileMode::Modified(ref path) => {
            println!("Updated: {}", path);
            vec![ft_api::bulk_update::Action::Updated {
                id,
                content: file.raw_content()?,
            }]
        }

        crate::types::FileMode::Deleted(ref path) => {
            println!("Deleted: {}", path);
            // TODO: find all parent directories and append them here
            vec![ft_api::bulk_update::Action::Deleted { id }]
        }
    })
}

// fn all_parents(dir: &str) -> crate::Result<Vec<ft_api::bulk_update::Action>>{
//
// }
