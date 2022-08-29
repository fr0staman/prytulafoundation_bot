use teloxide::{
    payloads::SendMessageSetters,
    requests::{Request, Requester},
    respond,
    types::{Message, User},
    utils::{command::BotCommands, html::user_mention_or_link},
    RequestError,
};

use crate::{db, types::MyBot};

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Admin commands")]
pub enum UserCommands {
    #[command(description = "User help")]
    Help,
}

pub async fn filter_new_users(msg: Message, new_members: Vec<User>, bot: MyBot) -> Result<(), RequestError> {
    let message_text = db::get_hi_msg().await;
    let real_message_text = message_text.unwrap();
    let user = new_members.get(0).unwrap();
    let formatted_message_text = real_message_text.replace("{}", &user_mention_or_link(&user));

    bot.send_message(msg.chat.id, formatted_message_text)
        .reply_to_message_id(msg.id)
        .send()
        .await?;
    respond(())
}

pub async fn filter_user_commands(msg: Message, bot: MyBot, cmd: UserCommands) -> Result<(), RequestError> {
    let text = match cmd {
        UserCommands::Help => db::get_help_msg().await.unwrap(),
    };
    bot.send_message(msg.chat.id, text).await?;
    respond(())
}
