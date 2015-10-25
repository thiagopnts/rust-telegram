

struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<String>>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    selective: Option<bool>,
}
