
use super::photo_size::PhotoSize;
use std::vec::Vec;

use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct UserProfilePhotos {
    total_count: u32,
    photos: Vec<Vec<PhotoSize>>,
}
