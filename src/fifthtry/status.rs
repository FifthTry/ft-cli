use crate::types::FTResult;
use reqwest;

pub fn call(authcode: &str) -> FTResult<String> {
    let url = format!(
        "https://www.fifthtry.com/api/sync-status/?auth_code={}",
        authcode
    );

    #[derive(Deserialize)]
    struct Status {
        last_synced_hash: String,
    }

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?;

    let status: Status = response.json()?;

    Ok(status.last_synced_hash)
}
