
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
struct ForceReply {
    force_reply: bool,
    selective: Option<bool>,
}
