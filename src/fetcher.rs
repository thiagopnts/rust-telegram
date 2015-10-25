
use std::io::Read;
use hyper::{Error, Client};
use hyper::client::Response;
use rustc_serialize::json::{Json, BuilderError};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct TelegramError(Json, Json);

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            client: Client::new(),
        }
    }
    pub fn get(self, url: String) -> Result<Json, TelegramError> {
        if let Ok(mut res) = self.client.get(&url).send() {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            let json = Json::from_str(&body).unwrap();
            if let Json::Object(obj) = json {
                if let &Json::Boolean(true) = obj.get("ok").unwrap() {
                    Ok(obj.get("result").unwrap().clone())
                } else {
                    Err(TelegramError(
                        obj.get("description").unwrap().clone(),
                        obj.get("error_code").unwrap().clone()
                    ))
                }
            } else {
                Err(TelegramError(Json::U64(000u64), Json::String("Unknown error".to_string())))
            }
        } else {
                Err(TelegramError(Json::U64(000u64), Json::String("Unknown error".to_string())))
        }
    }
}
