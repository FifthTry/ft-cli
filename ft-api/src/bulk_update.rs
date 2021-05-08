#[derive(Serialize)]
struct BulkUpdateInput {
    collection: String,
    auth_code: String,
    current_hash: String,
    new_hash: String,
    repo: String,
    files: Vec<Action>,
}

#[derive(Serialize)]
struct File {
    id: String,
    content: String,
}

#[derive(Deserialize)]
pub enum BulkUpdateError {
    InvalidAuthCode,
    RepoNotFound,
    CollectionNotFound,
    InvalidFileName(String),
    BadFTD(String),
    NoPermission(String),
}

pub fn bulk_update(
    collection: &str,
    current_hash: &str,
    new_hash: &str,
    repo: &str,
    files: Vec<Action>,
    auth_code: &str,
) -> crate::Result<()> {
    let url = "/testuser/index/~/bulk-update/";

    let update = BulkUpdateInput {
        collection: collection.trim().to_string(),
        auth_code: auth_code.trim().to_string(),
        current_hash: current_hash.trim().to_string(),
        new_hash: new_hash.trim().to_string(),
        repo: repo.trim().to_string(),
        files,
    };

    #[derive(Serialize)]
    struct UpdatedWrapper {
        data: BulkUpdateInput,
    }

    let update = UpdatedWrapper { data: update };

    let response: crate::api::ApiResponse<crate::sync_status::Status> =
        crate::api::post(&url, serde_json::to_value(update)?.to_string())?;

    if !response.success {
        return Err(crate::error::Error::ResponseError(
            response
                .error
                .map(|x| x.error)
                .unwrap_or_else(|| "".to_string()),
        )
        .into());
    }

    Ok(())
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Action {
    Updated { id: String, content: String },
    Added { id: String, content: String },
    Deleted { id: String },
}
