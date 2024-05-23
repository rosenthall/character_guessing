use crate::handler::CommandContext;
use config::CONFIG;
use database::get_winning_user_ids;
use log::trace;
use teloxide::{
    payloads::{GetChatMember, SendMessageSetters},
    prelude::UserId,
    requests::{JsonRequest, Requester},
};

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    //If user id is not in admin list - Do nothing
    if !CONFIG.clone().telegram.telegram_admin_ids.contains(&ctx.telegram_user.id.0.to_string()) {
        trace!("User {} is not in admin list!", &ctx.telegram_user.id.0.to_string());
        return Ok(());
    }

    let mut message: String =
        String::from("Вот люди которые справились с угадыванием сегодняшнего персонажа : \n ");
    let winners_list = {
        let mut users = vec![];
        let requests = get_winning_user_ids(&ctx.con)
            .unwrap()
            .iter()
            .map(|i| UserId(*i))
            .map(|i| ctx.bot.get_chat_member(ctx.msg.chat.id, i))
            .collect::<Vec<JsonRequest<GetChatMember>>>();

        for request in requests {
            let member = request.await.unwrap();
            users.push(member.user);
        }

        users
    };

    for winner in winners_list {
        message.insert_str(
            message.len(),
            &format!("{} ", &winner.mention().unwrap_or(winner.first_name)),
        );
    }

    let _ = ctx.bot.send_message(ctx.msg.chat.id, message).reply_to_message_id(ctx.msg.id).await;

    Ok(())
}
