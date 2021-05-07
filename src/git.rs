pub enum FileMode {
    Deleted(String),
    Renamed(String, String),
    Added(String),
    Modified(String),
}
