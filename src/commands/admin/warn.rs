use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
        Args
    },
    model::channel::Message,
    prelude::Context
};
use crate::database::ConnectionPool;
use crate::database::models::{Case, CaseType, GuildSettings};

use eyre::anyhow;
use tracing::{info, debug};

/*
* TODO:// Fix embed mentions
* for some reason giving now reason to the command doesn't send anything.
*/

#[command]
#[only_in(guild)]
// #[min_args(3)]
#[usage("warn <userMention> [reason]")]
pub async fn warn(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Error fetching guild data"))?;
    let guild_settings = GuildSettings::fetch_one(guild_id.to_string(), pool).await.unwrap();

    if args.is_empty() {
        msg.channel_id.say(&ctx.http, "You have to provide me who to warn and a reason for it.").await?;
        return Ok(())
    }

    if msg.mentions.is_empty() {
        msg.channel_id.say(&ctx.http, "Ummm... You didn't mention who to warn tho").await?;
        return Ok(())
    }

    if msg.mentions.len() > 1 {
        msg.channel_id.say(&ctx.http, "Dude I'm a bot but I can't warn two people at the same time.").await?;
        return Ok(())
    }

    let user_to_warn = &msg.mentions[0];
    let reason = args.advance().rest();

    debug!("{}", format!("{:?}", String::from(reason)));

    info!("Trying to insert a case!");
    let dbres = Case::insert_new(user_to_warn.id.to_string(), msg.author.id.to_string(), Option::from(CaseType::Warn), Option::from(String::from(reason)), &pool).await?;

    return if dbres.rows_affected() > 0 {

        if let Some(cases_channel) = guild_settings.case_channel {
            let cc = cases_channel.parse::<u64>().unwrap();
            let c = guild_id.channels(&ctx.http).await?;
            let c = c.values().find(|c| c.id == cc);

            if let Some(channel) = c {
                channel.send_message(&ctx.http, |m|
                    m.embed(|e|
                        e
                            .title(format!("<@{}> has been warned", user_to_warn.id.to_string()))
                            .description("He/She did fucky wonky stuff")
                            .field("Reason", reason, true)
                            .field("Warned by", msg.author.id, true)
                    )
                ).await?;
            }
            msg.channel_id.say(&ctx.http, "*insert I'm trying meme*").await?;
        }
    Ok(())
    } else {
        msg.channel_id.say(&ctx.http, "*insertion failed. press f to pay respects*").await?;
        Ok(())
    }

}