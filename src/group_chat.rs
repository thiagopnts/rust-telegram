
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct GroupChat {
    id: u64,
    title: String,
}
