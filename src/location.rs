
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    longitude: f32,
    lagitude: f32,
}
