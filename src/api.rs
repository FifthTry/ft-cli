use reqwest;

struct BulkUpdate {
    collection: String,
    auth_code: String,
    current_hash: String,
    new_hash: String,
    repo: String,
    files: Vec<File>,
}

struct File {
    id: String,
    content: String,
}

fn bulk_update(
    collection: &str,
    auth_code: &str,
    current_hash: &str,
    new_hash: &str,
    repo: &str,
    files: Vec<(String, String)>,
) {
    let url = "https://www.fifthtry.com/api/bulk-update/";
    let files = files
        .iter()
        .map(|(id, content)| File {
            id: id.into_string(),
            content: content.into_string(),
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
    //let response = reqwest::Client::new().post(url).json(update).send().await?;

    todo!()
}
