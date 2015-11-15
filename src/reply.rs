
use std::collections::BTreeMap;
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub enum Reply {
    ReplyKeyboardHide(bool, Option<bool>),
    ForceReply(bool, Option<bool>),
    ReplyKeyboardMarkup(Vec<Vec<String>>, Option<bool>, Option<bool>, Option<bool>),
}
