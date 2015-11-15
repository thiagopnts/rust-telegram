

use super::user::User;
use super::group_chat::GroupChat;
use super::video::Video;
use super::audio::Audio;
use super::photo_size::PhotoSize;
use super::sticker::Sticker;
use super::contact::Contact;
use super::location::Location;
use super::document::Document;

use std::collections::BTreeMap;
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message_id: u64,
    from: User,
    date: u64,
    chat: User,
    forward_from: Option<User>,
    forward_date: Option<u64>,
    reply_to_message: Option<Box<Message>>,
    text: String,
    audio: Option<Audio>,
    document: Option<Document>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
    video: Option<Video>,
    contact: Option<Contact>,
    location: Option<Location>,
    new_chat_participant: Option<User>,
    left_chat_participant: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
}

impl Message {
    pub fn new(obj: Value) -> Message {
        let json_str = serde_json::to_string(&obj).unwrap();
        println!("debug: {:?}", json_str);
        serde_json::from_str(&json_str).unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Chat {
    User,
    GroupChat,
}
