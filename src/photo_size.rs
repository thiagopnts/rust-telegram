
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    id: String,
    width: u32,
    height: u32,
    file_size: Option<u64>,
}
