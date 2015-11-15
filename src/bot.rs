

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

    pub fn send_message(self, chat_id: &str, text: &str)
            -> Result<Message, TelegramError> {
        let query_string = format!("text={}&chat_id={}", text, chat_id);

        match self.get_with_args("sendMessage", &query_string) {
            Ok(json) => Ok(Message::new(json)),
            Err(error) => Err(error),
        }
    }

    pub fn send_message_with_options(
        self,
        chat_id: String,
        text: String,
        parse_mode: Option<String>,
        disable_web_page_preview: Option<bool>,
        reply_to_message_id: Option<u64>,
        reply_markup: Option<Reply>,
        ) -> Result<Message, TelegramError>
    {
        let mut query_string = String::new();

        query_string.push_str(&format!("text={}&chat_id={}", text, chat_id));

        if parse_mode.is_some() {
            query_string.push_str(&format!("&parse_mode={}", parse_mode.unwrap()));
        }

        if disable_web_page_preview.is_some() {
            query_string.push_str(&format!("&disable_web_page_preview={}", disable_web_page_preview.unwrap()));
        }

        if reply_to_message_id.is_some() {
            query_string.push_str(&format!("&reply_to_message_id={}", reply_to_message_id.unwrap()));
        }

        if reply_markup.is_some() {
            query_string.push_str(&format!("&reply_markup={:?}", reply_markup.unwrap()));
        }

        match self.get_with_args("sendMessage", &query_string) {
            Ok(json) => Ok(Message::new(json)),
            Err(error) => Err(error),
        }
    }

    fn get(self, method: &str) -> Result<Value, TelegramError> {
        let url = format!("{}{}/{}", API_ENDPOINT, self.token, method);
        self.fetcher.get(url)
    }

    fn get_with_args(self, method: &str, args: &str) -> Result<Value, TelegramError> {
        let url = format!("{}{}/{}?{}", API_ENDPOINT, self.token, method, args);
        self.fetcher.get(url)
    }
}

