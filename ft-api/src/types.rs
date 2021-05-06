
pub enum FileMode {
    Deleted(String),
    Renamed(String, String),
    Added(String),
    Modified(String)
}


#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Action {
    Updated { id: String, content: String},
    Added {id: String, content: String},
    Deleted { id: String}
}
