#[derive(serde_derive::Serialize)]
struct BulkUpdateInput {
    collection: String,
    auth_code: String,
    current_hash: String,
    new_hash: String,
    repo: String,
    files: Vec<Action>,
}

#[derive(serde_derive::Serialize)]
struct File {
    id: String,
    content: String,
}

pub enum Error {
    RealmClientError(realm_client::Error),
    ContentMismatch { id: String },
}

#[allow(clippy::too_many_arguments)]
pub fn bulk_update(
    collection: &str,
    current_hash: &str,
    new_hash: &str,
    repo: &str,
    files: Vec<Action>,
    auth_code: &str,
    platform: String,
    client_version: String,
) -> realm_client::Result<()> {
    let url = format!("/{}/~/bulk-update/", collection);

    let update = BulkUpdateInput {
        collection: collection.trim().to_string(),
        auth_code: auth_code.trim().to_string(),
        current_hash: current_hash.trim().to_string(),
        new_hash: new_hash.trim().to_string(),
        repo: repo.trim().to_string(),
        files,
    };

    #[derive(serde_derive::Serialize)]
    struct UpdatedWrapper {
        data: BulkUpdateInput,
        platform: String,
        client_version: String,
    }

    realm_client::action::<crate::sync_status::Status, _>(
        &url,
        UpdatedWrapper {
            data: update,
            platform,
            client_version,
        },
        Some("bulk_update".to_string()),
    )?;
    Ok(())
}

#[derive(serde_derive::Serialize, Debug)]
#[serde(tag = "type")]
pub enum Action {
    Updated { id: String, content: String },
    Added { id: String, content: String },
    Deleted { id: String },
}

impl Action {
    fn id(&self) -> &str {
        match self {
            Self::Updated { id, .. } => id,
            Self::Added { id, .. } => id,
            Self::Deleted { id } => id,
        }
    }

    fn is_updated(&self) -> bool {
        matches!(self, Self::Updated { .. })
    }

    fn is_added(&self) -> bool {
        matches!(self, Self::Added { .. })
    }

    fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted { .. })
    }
}

fn digest(actions: Vec<Action>) -> Vec<Action> {
    // more than one Updated with the same id, ensure each content is exactly same and merge
    // into one
    //
    let updated: Vec<_> = actions
        .iter()
        .filter(|x| x.is_updated())
        .filter_map(|x| {
            if let Action::Updated { id, content } = x {
                Some((id, content))
            } else {
                None
            }
        })
        .collect();

    let updated_map: std::collections::HashMap<_, _> =
        updated.clone().into_iter().map(|x| x).collect();

    for (id, content) in updated.iter() {
        if let Some(c) = updated_map.get(id) {
            if !c.eq(content) {
                println!("Updated action: content is not same for id: {}", id)
                // return error
            }
        }
    }

    // TODO: collect unique updated ID's

    // more than one Added with the same id, ensure each content is exactly same and merge
    // into one
    //

    let added: Vec<_> = actions
        .iter()
        .filter(|x| x.is_added())
        .filter_map(|x| {
            if let Action::Added { id, content } = x {
                Some((id, content))
            } else {
                None
            }
        })
        .collect();

    let added_map: std::collections::HashMap<_, _> = added.clone().into_iter().map(|x| x).collect();

    for (id, content) in added.iter() {
        if let Some(c) = added_map.get(id) {
            if !c.eq(content) {
                println!("Added action: content is not same for id: {}", id)
                // return error
            }
        }
    }
    // TODO: collect unique added ID's

    // if more than one Deleted with the same id, merge into one
    //

    let deleted: Vec<_> = actions
        .iter()
        .filter(|x| x.is_deleted())
        .filter_map(|x| {
            if let Action::Deleted { id } = x {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    let deleted: std::collections::HashMap<_, _> =
        deleted.clone().into_iter().map(|x| (x, true)).collect();

    // TODO: collect unique deleted ID's

    // if both Updated and Added have same id, ensure content matches and than merge Updated
    //

    // if something is added and deleted, return Error::AddedAndDeleted {id: ""}
    //
    // if content is both Updated and Deleted, only send Deleted
    //
    // Note: If content mismatches, we will return Error::ContentMismatch
    vec![]
}
