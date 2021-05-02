use crate::types::FTResult;

pub fn call(authcode: &str) -> FTResult<String> {
    let url = format!(
        "http://127.0.0.1:3000/api/sync-status/?auth_code={}",
        authcode
    );

    #[derive(Deserialize)]
    struct Status {
        last_synced_hash: String,
        last_updated_on: String,
    }

    let response: crate::fifthtry::api::ApiResponse<Status>  = crate::fifthtry::api::get(&url)?;

    if !response.success || response.result.is_none() {
        return Err(crate::error::FTSyncError::ResponseError(
            response.error.map(|x| x.error.to_string())
                .unwrap_or("".to_string())).into())
        //crate::error::FTSyncError::ResponseError("response in none".to_string()).into()
    }

    let resp = response.result.unwrap();

    Ok(resp.last_synced_hash)
}
