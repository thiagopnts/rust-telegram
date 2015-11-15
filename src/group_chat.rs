
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupChat {
    id: u64,
    title: String,
}
