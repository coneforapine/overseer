use crate::database::models::{Case, CaseType, GuildSettings};
use crate::database::ConnectionPool;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use eyre::anyhow;
use serenity::model::id::ChannelId;

#[command]
#[only_in(guild)]
#[usage("warn <userMention> [reason]")]
#[required_permissions(ADMINISTRATOR)]
pub async fn warn(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let guild_id = msg
        .guild_id
        .ok_or_else(|| anyhow!("Error fetching guild data"))?;
    let guild_settings = GuildSettings::fetch_one(guild_id.to_string(), pool)
        .await
        .unwrap();

    if args.is_empty() {
        msg.channel_id
            .say(
                &ctx.http,
                "You have to provide me who to warn and a reason for it.",
            )
            .await?;
        return Ok(());
    }

    if msg.mentions.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Ummm... You didn't mention who to warn tho")
            .await?;
        return Ok(());
    }

    if msg.mentions.len() > 1 {
        msg.channel_id
            .say(
                &ctx.http,
                "Dude I'm a bot but I can't warn two people at the same time.",
            )
            .await?;
        return Ok(());
    }

    let user_to_warn = &msg.mentions[0];
    let mut reason = args.advance().rest();
    if reason.is_empty() {
        reason = "No reason provided";
    }

    let case_number = Case::insert_new(
        user_to_warn.id.to_string(),
        msg.author.id.to_string(),
        CaseType::Warn,
        String::from(reason),
        &pool,
    ).await?;

    if let Some(cases_channel_id) = guild_settings.case_channel {

        let channel = ChannelId(cases_channel_id.parse::<u64>().unwrap());

        channel.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a| {
                        a.name(format!("{}#{} has been warned! | CASE #{}", user_to_warn.name, user_to_warn.discriminator, case_number))
                })
                    .description(format!("User ID {} | Event time: //todo for now", user_to_warn.id))
                    .field("Reason", reason, true)
                    .field("Warned by", format!("<@!{}>", msg.author.id), true)
                    .footer(|f| {
                        f.text(format!("Use jk!case update {} to change reason!", case_number))
                    })
            })
        }).await?;
        msg.channel_id.say(&ctx.http, format!("{}#{} warned successfully!", user_to_warn.name, user_to_warn.discriminator)).await?;
    }
    Ok(())
}