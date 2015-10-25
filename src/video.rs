
use super::photo_size::PhotoSize;

use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct Video {
    file_id: String,
    width: u32,
    height: u32,
    duration: u64,
    thumb: Option<PhotoSize>,
    mime_type: Option<String>,
    file_size: Option<u64>,
}
