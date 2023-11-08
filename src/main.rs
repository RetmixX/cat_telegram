mod load_config;
mod buttons;
mod utils;
mod messages;

use teloxide::Bot;
use teloxide::prelude::*;
use crate::buttons::handle_press_buttons;
use crate::messages::message_handler;

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    let token = load_config::load_token();

    let bot = Bot::new(token);
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(handle_press_buttons));
    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}
