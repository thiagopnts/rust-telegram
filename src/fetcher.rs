
use std::io::Read;
use hyper::{Error, Client};
use hyper::client::Response;
use serde_json;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct TelegramError(Value, Value);

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            client: Client::new(),
        }
    }
    pub fn get(self, url: String) -> Result<Value, TelegramError> {
        if let Ok(mut res) = self.client.get(&url).send() {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            let value: Value = serde_json::from_str(&body).unwrap();
            let obj = value.as_object();
            if let Some(obj) = obj {
                if let &Value::Bool(true) = obj.get("ok").unwrap() {
                    Ok(obj.get("result").unwrap().clone())
                } else {
                    Err(TelegramError(
                        obj.get("description").unwrap().clone(),
                        obj.get("error_code").unwrap().clone()
                    ))
                }
            } else {
                Err(TelegramError(Value::U64(000u64), Value::String("Unknown error".to_string())))
            }

        } else {
                Err(TelegramError(Value::U64(000u64), Value::String("Unknown error".to_string())))
        }
    }
}
