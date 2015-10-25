mod photo_size;

use photo_size::PhotoSize;
use std::vec::Vec;

struct UserProfilePhotos {
    total_count: u32,
    photos: Vec<Vec<PhotoSize>>,
}
