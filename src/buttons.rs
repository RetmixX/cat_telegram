use std::path::Path;
use teloxide::Bot;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use crate::HandlerResult;
use teloxide::prelude::*;
use crate::utils::get_random_file;

pub async fn handle_press_buttons(query: CallbackQuery, bot: Bot) -> HandlerResult {
    let input_button = query.data.unwrap();
    let user_id = query.from.id;
    bot.answer_callback_query(query.id).await?;
    match input_button.as_str() {
        "meow" => {
            let path_cat = &get_random_file();
            bot.send_photo(user_id,
                           InputFile::file(Path::new(&get_random_file())))
                .reply_markup(make_buttons()).await?
        }
        _ => bot.send_message(user_id, "Action not found").reply_markup(make_buttons())
            .await?
    };

    Ok(())
}

pub fn make_buttons() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::default()
        .append_row(vec![InlineKeyboardButton::callback("Meow", "meow")])
}