
use std::collections::BTreeMap;
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u64,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    #[serde(rename(json="type"))]
    type_: Option<String>,
}

impl User {
    pub fn new(obj: Value) -> User {
        let json_str = serde_json::to_string(&obj).unwrap();
        serde_json::from_str(&json_str).unwrap()
    }
}
