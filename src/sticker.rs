
use super::photo_size::PhotoSize;

use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    file_id: String,
    width: u32,
    height: u32,
    thumb: Option<PhotoSize>,
    file_size: Option<u64>,
}
