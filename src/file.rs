
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct File {
    file_id: String,
    file_size: Option<u64>,
    file_path: Option<String>,
}
