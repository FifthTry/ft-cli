use crate::types::FTResult;
pub use crate::*;

#[derive(Serialize)]
struct BulkUpdateInput {
    collection: String,
    auth_code: String,
    current_hash: String,
    new_hash: String,
    repo: String,
    files: Vec<File>,
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

pub fn call(
    collection: &str,
    current_hash: &str,
    new_hash: &str,
    repo: &str,
    files: Vec<(String, String)>,
    auth_code: &str,
) -> FTResult<()> {
    let url = "http://127.0.0.1:3000/testuser/index/~/bulk-update/?realm_mode=api";
    let files = files
        .iter()
        .map(|(id, content)| File {
            id: id.to_string(),
            content: content.to_string(),
        })
        .collect();

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

    let update = UpdatedWrapper {
        data: update
    };

    let response: crate::fifthtry::api::ApiResponse<std::collections::HashMap<String, String>>  =
        crate::fifthtry::api::post(&url, serde_json::to_value(update)?.to_string())?;

    if !response.success {
        return Err(crate::error::FTSyncError::ResponseError(
            response.error.map(|x| x.error.to_string())
                .unwrap_or("".to_string())).into())
    }

    Ok(())
}
