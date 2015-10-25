
use super::photo_size::PhotoSize;

use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct Sticker {
    file_id: String,
    width: u32,
    height: u32,
    thumb: Option<PhotoSize>,
    file_size: Option<u64>,
}
