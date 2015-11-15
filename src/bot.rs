

use std::io::Read;

use super::fetcher::{Fetcher, TelegramError};
use super::user::User;
use super::message::Message;
use super::reply::Reply;

use hyper::{Error, Client};
use hyper::client::Response;
use std::collections::BTreeMap;

use serde_json::Value;

static API_ENDPOINT: &'static str = "https://api.telegram.org/bot";

pub struct Bot {
    token: String,
    fetcher: Fetcher,
}

impl Bot {
    pub fn new(token: String) -> Bot {
        Bot {
            token: token,
            fetcher: Fetcher::new(),
        }
    }

    pub fn me(self) -> Result<User, TelegramError> {
        match self.get("getMe") {
            Ok(json) => Ok(User::new(json)),
            Err(error) => Err(error),
        }
    }

    fn get(self, method: &str) -> Result<Json, TelegramError> {
        let url = format!("{}{}/{}", API_ENDPOINT, self.token, method);
        self.fetcher.get(url)
    }
}

