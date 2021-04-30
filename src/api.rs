use crate::types::FTResult;
use reqwest;

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

#[derive(DeSerialize)]
struct BulkUpdateOutput {
    success: bool,
    errors: Vec<BulkUpdateError>,
}

enum BulkUpdateError {
    InvalidAuthCode,
    RepoNotFound,
    CollectionNotFound,
    InvalidFileName(String),
    BadFTD(String),
    NoPermission(String),
}

fn bulk_update(
    collection: &str,
    current_hash: &str,
    new_hash: &str,
    repo: &str,
    files: Vec<(String, String)>,
    auth_code: &str,
) -> FTResult<BulkUpdateOutput> {
    let url = "https://www.fifthtry.com/api/bulk-update/";
    let files = files
        .iter()
        .map(|(id, content)| File {
            id: id.to_owned(),
            content: content.to_owned(),
        })
        .collect();

    let update = BulkUpdate {
        collection: collection.to_string(),
        auth_code: auth_code.to_string(),
        current_hash: current_hash.to_string(),
        new_hash: new_hash.to_string(),
        repo: repo.to_string(),
        files: files,
    };

    let client = reqwest::blocking::Client::new();
    let response = reqwest::Client::new().post(url).json(&update).send();

    OK(response)
}

fn status(authcode: &str) -> FTResult<()> {
    let url = "https://www.fifthtry.com/api/sync-status/";

    todo!()
}
