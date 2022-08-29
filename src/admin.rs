use teloxide::requests::Requester;
use teloxide::respond;
use teloxide::utils::html::user_mention_or_link;
use teloxide::{types::Message, utils::command::BotCommands, RequestError};

use crate::db;
use crate::helpers::*;
use crate::types::*;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Admin commands")]
pub enum AdminCommands {
    #[command(description = "Get or set hello message to user")]
    Hello,
    #[command(description = "Get or set /help message to user")]
    Help,
    #[command(description = "Start of admin")]
    Start,
}

pub async fn filter_admin_commands(msg: Message, bot: MyBot, cmd: AdminCommands) -> Result<(), RequestError> {
    let text = match cmd {
        AdminCommands::Hello => {
            let text = msg.text().unwrap();

            let (_, second) = split_half(text);
            if second == "" {
                let result = db::get_hi_msg().await.unwrap();
                result.replace("{}", &user_mention_or_link(&msg.from().unwrap()))
            } else {
                let _ = db::set_hi_msg(second.to_string()).await;

                "Готово, привітання виставлене!".to_string()
            }
        },
        AdminCommands::Start => "Вітаю, Адмін!".to_string(),
        AdminCommands::Help => {
            let text = msg.text().unwrap();

            let (_, second) = split_half(text);
            if second == "" {
                let result = db::get_help_msg().await.unwrap();
                result.replace("{}", &user_mention_or_link(&msg.from().unwrap()))
            } else {
                let _ = db::set_help_msg(second.to_string()).await;

                "Готово, текст для команди /help виставлено!".to_string()
            }
        },
    };
    bot.send_message(msg.chat.id, text).await?;
    respond(())
}
