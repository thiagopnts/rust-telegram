
use std::collections::BTreeMap;
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub enum Reply {
    ReplyKeyboardHide(bool, Option<bool>),
    ForceReply(bool, Option<bool>),
    ReplyKeyboardMarkup(Vec<Vec<String>>, Option<bool>, Option<bool>, Option<bool>),
}
