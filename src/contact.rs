
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<u32>,
}
