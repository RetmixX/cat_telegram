use std::path::Path;
use rand::prelude::SliceRandom;
use teloxide::Bot;
use teloxide::macros::BotCommands;
use teloxide::prelude::{Requester, ResponseResult};
use teloxide::types::{InputFile, Message};
use teloxide::repls::CommandReplExt;

#[tokio::main]
async fn main() {
    let token = "6426435379:AAGOIYko2rXwTHRbD9JHVJ7cgTtk16EzP1g";
    let bot = Bot::new(token);
    Command::repl(bot, send_cat).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Meow
}

async fn send_cat(bot: Bot, message: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Meow => bot.send_photo(message.chat.id, InputFile::file(Path::new(&get_random_file()))).await?,
        _=> bot.send_message(message.chat.id, "Undefined command").await?
    };

    Ok(())
}

fn get_random_file() -> String {
    let path = "cats/";

    let status_code_array = [
        100, 101, 102, 103,
        200, 201, 202, 203, 204, 206, 207,
        300, 301, 302, 303, 304, 305, 307, 308,
        400, 401, 402, 403, 404, 405, 406, 407, 408, 409,
        410, 411, 412, 413, 414, 415, 416, 417, 418, 420,
        421, 422, 423, 424, 425, 426, 428, 429, 431, 444,
        450, 451, 497, 498, 499,
        500, 501, 502, 503, 504, 506, 507, 508, 509, 510,
        511, 521, 522, 523, 525, 530, 599
    ];
    let random_pic = status_code_array.choose(&mut rand::thread_rng()).unwrap();

    format!("{path}/{random_pic}.jpg")
}
