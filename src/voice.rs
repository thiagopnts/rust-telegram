
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct Voice {
    file_id: String,
    duration: u64,
    mime_type: Option<String>,
    file_size: Option<u64>,
}
