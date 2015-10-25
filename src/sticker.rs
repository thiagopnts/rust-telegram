
mod photo_size;

use photo_size::PhotoSize;

struct Sticker {
    file_id: String,
    width: u32,
    height: u32,
    thumb: Option<PhotoSize>,
    file_size: Option<u64>,
}
