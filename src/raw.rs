pub fn handle(
    file: crate::FileMode,
    root_dir: &str,
    collection: &str,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let id = file.id_with_extension(root_dir, collection);

    Ok(match file {
        crate::types::FileMode::Created(_) => {
            println!("Created: {}", id.as_str());
            // TODO: find all parent directories and append them here
            // let all = all_parents();
            vec![ft_api::bulk_update::Action::Added {
                id,
                content: file.raw_content()?,
            }]
        }

        crate::types::FileMode::Modified(_) => {
            println!("Updated: {}", id.as_str());
            vec![ft_api::bulk_update::Action::Updated {
                id,
                content: file.raw_content()?,
            }]
        }

        crate::types::FileMode::Deleted(_) => {
            println!("Deleted: {}", id.as_str());
            // TODO: find all parent directories and append them here
            vec![ft_api::bulk_update::Action::Deleted { id }]
        }
    })
}

// fn all_parents(dir: &str) -> crate::Result<Vec<ft_api::bulk_update::Action>>{
//
// }
