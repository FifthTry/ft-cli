#[derive(serde_derive::Deserialize, Debug)]
pub struct Status {
    pub last_synced_hash: String,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub last_updated_on: chrono::DateTime<chrono::Utc>,
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use chrono::TimeZone;

    // use our visitor to deserialize an `ActualValue`
    let v: i64 = serde::de::Deserialize::deserialize(deserializer)?;

    Ok(chrono::Utc.timestamp_millis(v))
}

// TODO: define ActionError here and return actual errors that sync status can throw.

pub fn sync_status(collection: &str, auth_code: &str) -> realm_client::Result<Status> {
    realm_client::page(
        &format!("/{}/~/sync-status/", collection),
        maplit::hashmap! {"auth_code" => auth_code},
        Some("status".to_string()),
    )
}
