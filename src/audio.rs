use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    file_id: String,
    duration: u64,
    performer: Option<String>,
    title: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u64>,
}
