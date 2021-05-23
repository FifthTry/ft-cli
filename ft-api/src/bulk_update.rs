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

fn digest(actions: &[Action]) -> Vec<Action> {
    // more than one Updated with the same id, ensure each content is exactly same and merge
    // into one
    //
    // more than one Added with the same id, ensure each content is exactly same and merge
    // into one
    //
    // if both Updated and Added have same id, ensure content matches and than merge Updated
    //
    // if more than one Deleted with the same id, merge into one
    //
    // if something is added and deleted, return Error::AddedAndDeleted {id: ""}
    //
    // if content is both Updated and Deleted, only send Deleted
    //
    // Note: If content mismatches, we will return Error::ContentMismatch
    vec![]
}
