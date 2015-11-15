
use super::photo_size::PhotoSize;

use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    file_id: String,
    width: u32,
    height: u32,
    duration: u64,
    thumb: Option<PhotoSize>,
    mime_type: Option<String>,
    file_size: Option<u64>,
}
