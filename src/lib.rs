#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

extern crate hyper;

mod audio;
mod bot;
mod contact;
mod document;
mod fetcher;
//mod file;
mod group_chat;
mod location;
mod message;
mod photo_size;
mod reply;
mod sticker;
mod user;
//mod user_profile_photos;
mod video;
//mod voice;

pub use self::bot::Bot;

