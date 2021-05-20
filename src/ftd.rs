pub fn handle(
    file: crate::FileMode,
    root_dir: &str,
    collection: &str,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    if file.extension() != "ftd" {
        return Ok(vec![]);
    }

    let id = file.id(root_dir, collection);

    Ok(match file {
        crate::types::FileMode::Added(ref path) => {
            println!("Added new: {}", path);
            vec![ft_api::bulk_update::Action::Added {
                id,
                content: file.content()?,
            }]
        }

        crate::types::FileMode::Modified(ref path) => {
            println!("Updated: {}", path);
            vec![ft_api::bulk_update::Action::Updated {
                id,
                content: file.content()?,
            }]
        }

        crate::types::FileMode::Deleted(ref path) => {
            println!("Deleted: {}", path);
            vec![ft_api::bulk_update::Action::Deleted { id }]
        }
    })
}
