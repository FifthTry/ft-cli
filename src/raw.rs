pub fn handle(
    root_tree: &crate::traverse::Node,
    file: crate::FileMode,
    root_dir: &str,
    collection: &str,
) -> crate::Result<Vec<ft_api::bulk_update::Action>> {
    let id = file.id_with_extension(root_dir, collection);

    Ok(match file {
        crate::types::FileMode::Created(ref file_path) => {
            println!("Created: {}", id.as_str());
            let all_parent = crate::traverse::ancestors(root_tree, file_path);
            let mut actions: Vec<ft_api::bulk_update::Action> = all_parent
                .into_iter()
                .filter(|x| !x.readme_exists())
                .map(|node| ft_api::bulk_update::Action::Updated {
                    id: node
                        .document_id(root_dir, collection)
                        .to_string_lossy()
                        .to_string(),
                    content: node.to_markdown(root_dir, collection),
                })
                .collect();

            actions.push(ft_api::bulk_update::Action::Added {
                id,
                content: file.raw_content()?,
            });
            actions
        }

        crate::types::FileMode::Modified(_) => {
            println!("Updated: {}", id.as_str());
            vec![ft_api::bulk_update::Action::Updated {
                id,
                content: file.raw_content()?,
            }]
        }

        crate::types::FileMode::Deleted(ref file_path) => {
            let all_parent = crate::traverse::ancestors(root_tree, file_path);
            let mut actions: Vec<ft_api::bulk_update::Action> = all_parent
                .into_iter()
                .filter(|x| !x.readme_exists())
                .map(|node| ft_api::bulk_update::Action::Updated {
                    id: node
                        .document_id(root_dir, collection)
                        .to_string_lossy()
                        .to_string(),
                    content: node.to_markdown(root_dir, collection),
                })
                .collect();
            println!("Deleted: {}", id.as_str());
            actions.push(ft_api::bulk_update::Action::Deleted { id });
            actions
        }
    })
}
