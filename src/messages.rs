use std::path::Path;
use teloxide::Bot;
use teloxide::types::{InputFile, Me, Message};
use crate::HandlerResult;
use teloxide::macros::BotCommands as Commands;
use teloxide::net::Download;
use teloxide::prelude::Requester;
use teloxide::utils::command::BotCommands;
use crate::utils::{check_creator, check_permission, get_file, get_random_file};
use teloxide::prelude::*;
use uuid::Uuid;
use crate::buttons::make_buttons;

pub async fn message_handler(bot: Bot, msg: Message, me: Me) -> HandlerResult {
    if let Some(text) = msg.text() {
        match Command::parse(text, me.username()) {
            Ok(Command::Start) => {
                bot.send_message(msg.chat.id, "Okay, start").reply_markup(make_buttons()).await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Action not found").await?;
            }
        }
    };

    if let Some(photo) = msg.photo() {
        if !check_permission(msg.chat.id.0 as u64) {
            bot.send_photo(msg.chat.id,
                           InputFile::file(get_file("cats/403.jpg")))
                .reply_markup(make_buttons()).await?;

            return Ok(());
        }
        let image = photo.to_vec().last().unwrap().clone();

        let path = bot.get_file(image.file.id).send().await
            .expect("Cannot get file").path;

        let mut file_save = tokio::fs::File::create(format!("cats/{}.jpg", Uuid::new_v4().to_string()))
            .await.expect("Cannot create file");

        bot.download_file(&path, &mut file_save).await.expect("Cannot save data to file");

        bot.send_message(msg.chat.id, format!("Success! Cat save and secure!")).await?;
    };
    Ok(())
}

#[derive(Commands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start
}