mod admin;
mod db;
mod helpers;
mod types;
mod user;

use std::env;

use dotenv::dotenv;
use teloxide::{
    prelude::*,
    types::{ParseMode, Update, UserId},
};

use crate::user::UserCommands;

#[derive(Clone)]
struct ConfigParameters {
    admin_ids: Vec<UserId>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    run().await;
}

async fn run() {
    let clean_env_admins = env::var("ADMINS").unwrap();
    let admins = clean_env_admins.split(",");
    let admin_ids = admins
        .collect::<Vec<_>>()
        .iter()
        .map(|e| UserId(u64::from(e.parse::<u64>().unwrap())))
        .collect();
    log::info!("Bot is starting...");
    let bot = Bot::from_env().parse_mode(ParseMode::Html).auto_send();

    let parameters = ConfigParameters { admin_ids };

    let handler = Update::filter_message()
        .branch(
            dptree::filter(|msg: Message, cfg: ConfigParameters| {
                msg.chat.is_private() && cfg.admin_ids.contains(&msg.from().unwrap().id)
            })
            .filter_command::<admin::AdminCommands>()
            .endpoint(admin::filter_admin_commands),
        )
        .branch(
            dptree::entry()
                .filter_command::<UserCommands>()
                .endpoint(user::filter_user_commands),
        )
        .branch(Message::filter_new_chat_members().endpoint(user::filter_new_users));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![parameters])
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler.
        .error_handler(LoggingErrorHandler::with_custom_text("Error with dispatcher"))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
