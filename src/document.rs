mod photo_size;

use photo_size::PhotoSize;

struct Document {
    file_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u64>,
}
