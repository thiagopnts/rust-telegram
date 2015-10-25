

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
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcDecodable, Debug)]
pub struct Message {
    id: u64,
    from: User,
    date: u64,
    chat: Chat,
    forward_from: Option<User>,
    forward_date: u64,
    reply_to_message: Box<Option<Message>>, // this should be a Box<Option<Message>> probably
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


#[derive(RustcDecodable, Debug)]
pub enum Chat {
    User,
    GroupChat,
}
