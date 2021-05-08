#[derive(Deserialize)]
pub struct Status {
    pub last_synced_hash: String,
    // TODO: use custom deserializer and convert its type to DateTime<Utc>
    pub last_updated_on: i64,
}

pub fn sync_status(
    collection: &str,
    auth_code: &str,
) -> crate::Result<(String, chrono::DateTime<chrono::Utc>)> /* TODO: return Status */ {
    use chrono::TimeZone;
    let url = format!("/{}/~/sync-status/", collection);

    let response: crate::api::ApiResponse<Status> =
        crate::api::get(&url, maplit::hashmap! {"auth_code" => auth_code})?;

    // TODO: abstract out this pattern into api::get() function
    if !response.success || response.result.is_none() {
        return Err(crate::error::Error::ResponseError(
            response
                .error
                .map(|x| x.error)
                .unwrap_or_else(|| "".to_string()),
        )
        .into());
    }

    let resp = response.result.unwrap(); // safe because we did error check

    Ok((
        resp.last_synced_hash,
        chrono::Utc.timestamp_millis(resp.last_updated_on),
    ))
}
