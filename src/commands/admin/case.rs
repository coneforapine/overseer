use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
        Args
    },
    model::channel::Message,
    prelude::Context
};

use crate::database::{
    ConnectionPool,
    models::Case
};
use serenity::model::id::UserId;
use crate::utils::match_case_enum;

#[command]
#[sub_commands(case_update)]
#[usage("case")]
#[only_in(guild)]
#[required_permissions(ADMINISTRATOR)]
pub async fn case(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "And the Pinecone said let there be case number").await?;
    }

    if let Some(case_number) = args.single::<i32>().ok() {
        if let Some(case) = Case::get_from_case_number(case_number, pool).await {
            let acted_user = UserId(case.user_id.parse::<u64>().unwrap()).to_user(&ctx.http).await?;
            let moderator = UserId(case.moderator_id.parse::<u64>().unwrap()).to_user(&ctx.http).await?;

            msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("The case of {}#{} | CASE #{}", acted_user.name, acted_user.discriminator, case_number))
                    .field("Reason", case.reason.unwrap(), true)
                    .field("Moderator", format!("<@{}>", moderator.id) , true)
                    .field("Action type", match_case_enum(case.case_type), true)
                })
            }).await?;
        } else {
            msg.channel_id.say(&ctx.http, "There is no case with that number. Are u sure?").await?;
        }
    } else {
        msg.channel_id.say(&ctx.http, "I need a case to see.").await?;
    }

    Ok(())
}

#[command("update")]
#[only_in("guilds")]
#[required_permissions(ADMINISTRATOR)]
#[usage("case update <number> <reason>")]
pub async fn case_update(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "You haven't said anything about case number and reason to change").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    if let Some(case_number) = args.single::<i32>().ok() {
        let reason = args.rest();
        if reason.is_empty() {
            msg.channel_id.say(&ctx.http, format!("It was at this moment that <@{}> knew. I need a reason.", msg.author.id)).await?;
            return Ok(())
        }

        let dbres = Case::update_reason(case_number, reason.to_string(), pool).await?;

        if dbres.rows_affected() == 0 {
            msg.channel_id.say(&ctx.http, format!("Failed to update reason for case #{}", case_number)).await?;
            return Ok(())
        }

        msg.channel_id.say(&ctx.http, format!("Successfully updated reason for case #{}", case_number)).await?;
    } else {
        msg.channel_id.say(&ctx.http, "I need the case number for that").await?;
    }
    Ok(())
}