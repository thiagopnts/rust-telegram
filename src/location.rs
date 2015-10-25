
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct Location {
    longitude: f32,
    lagitude: f32,
}
