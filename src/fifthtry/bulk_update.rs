use crate::types::FTResult;
use reqwest;
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
pub struct BulkUpdateOutput {
    pub success: bool,
    pub errors: Vec<BulkUpdateError>,
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
) -> FTResult<BulkUpdateOutput> {
    let url = "https://www.fifthtry.com/api/bulk-update/";
    let files = files
        .iter()
        .map(|(id, content)| File {
            id: id.to_string(),
            content: content.to_string(),
        })
        .collect();

    let update = BulkUpdateInput {
        collection: collection.to_string(),
        auth_code: auth_code.to_string(),
        current_hash: current_hash.to_string(),
        new_hash: new_hash.to_string(),
        repo: repo.to_string(),
        files,
    };

    let client = reqwest::blocking::Client::new();
    let response = client.post(url).json(&update).send()?;

    Ok(response.json()?)
}
