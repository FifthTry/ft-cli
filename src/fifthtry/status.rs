use crate::types::FTResult;
use chrono::TimeZone;

pub fn call(authcode: &str) -> FTResult<(String, chrono::DateTime<chrono::Utc>)> {
    let url = format!(
        "http://127.0.0.1:3000/a/b/~/sync-status/?auth_code={}&realm_mode=api",
        authcode
    );

    #[derive(Deserialize)]
    struct Status {
        last_synced_hash: String,
        last_updated_on: i64,
    }

    let response: crate::fifthtry::api::ApiResponse<Status>  = crate::fifthtry::api::get(&url)?;

    if !response.success || response.result.is_none() {
        return Err(crate::error::FTSyncError::ResponseError(
            response.error.map(|x| x.error)
                .unwrap_or("".to_string())).into())
    }

    let resp = response.result.unwrap();

    Ok((resp.last_synced_hash, chrono::Utc.timestamp_millis(resp.last_updated_on)))
}
