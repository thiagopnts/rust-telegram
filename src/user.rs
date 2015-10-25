use std::collections::BTreeMap;
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct User {
    id: u64,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
}

impl ToJson for User {
    fn to_json(&self) -> Json {
        let mut obj = BTreeMap::new();
        obj.insert("id".to_string(), self.id.to_json());
        obj.insert("first_name".to_string(), self.first_name.to_json());
        obj.insert("last_name".to_string(), self.last_name.to_json());
        obj.insert("username".to_string(), self.username.to_json());
        Json::Object(obj)
    }
}

impl User {
    pub fn new(obj: Json) -> User {
        let json_str = obj.to_string();
        json::decode(&json_str).unwrap()
    }
}
