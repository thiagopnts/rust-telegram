
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct PhotoSize {
    id: String,
    width: u32,
    height: u32,
    file_size: Option<u64>,
}
