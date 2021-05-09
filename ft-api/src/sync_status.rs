#[derive(Deserialize)]
pub struct Status {
    pub last_synced_hash: String,
    // TODO: use custom deserializer and convert its type to DateTime<Utc>
    pub last_updated_on: i64,
}

pub fn sync_status(
    collection: &str,
    auth_code: &str,
) -> crate::PageResult<(String, chrono::DateTime<chrono::Utc>)> /* TODO: return Status */ {
    use chrono::TimeZone;
    let url = format!("/{}/~/sync-status/", collection);

    let resp: crate::PageResult<Status> =
        crate::api::page(&url, maplit::hashmap! {"auth_code" => auth_code});

    let resp = resp?;

    Ok((
        resp.last_synced_hash,
        chrono::Utc.timestamp_millis(resp.last_updated_on),
    ))
}
