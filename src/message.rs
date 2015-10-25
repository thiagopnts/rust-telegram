mod user;
mod group_chat;

use user::User;
use group_chat::GroupChat;

struct Message {
    id: u64,
    from: User,
    date: u64,
    chat: Chat,
    forward_from: Option<User>,
    forward_date: u64,
    reply_to_message: Option<Message>, // this should be a Box<Option<Message>> probably
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

enum Chat {
    User,
    GroupChat,
}
