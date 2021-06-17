use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
        Args
    },
    model::channel::Message,
    prelude::Context
};

use tracing::{info, warn};
use eyre::anyhow;

use crate::{
    database::{ models, ConnectionPool },
    utils::parse_chan
};
use crate::database::models::GuildSettings;
use serenity::model::id::ChannelId;

#[command]
#[sub_commands(set_channel)]
#[only_in(guilds)]
#[min_args(1)]
#[required_permissions(ADMINISTRATOR)]
#[usage("announce <message>")]
pub async fn announce(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Error fetching guild data"))?;
    let guild_settings = models::GuildSettings::fetch_one(guild_id.to_string(), pool).await.unwrap();

    if let Some(ac) = guild_settings.announcement_channel {
        let channel = ChannelId(ac.parse::<u64>().unwrap());
        channel.say(&ctx.http, &args.message()).await?;
    }
    Ok(())
}

#[command("set")]
#[only_in(guilds)]
#[max_args(1)]
#[required_permissions(ADMINISTRATOR)]
#[usage("announce set <channel>")]
pub async fn set_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    if args.is_empty() {
        msg.channel_id.say(&ctx.http,"What channel? I don't see anything").await?;
        return Ok(())
    }

    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Error fetching guild id"))?;
    if let Some(channel_id) = parse_chan(
        &args.quoted()
            .current()
            .ok_or_else(|| anyhow!("Channel not found"))?
            .to_string(),
        Some(&guild_id),
        Some(&ctx)
    ).await {
        let dbres = GuildSettings::update_single_field(guild_id.to_string(),
                                                       String::from("announcement_channel"),
                                                       channel_id.to_string(),
                                                       pool
        ).await?;

        if dbres.rows_affected() > 0 {
            msg.channel_id.say(&ctx.http, format!("Success! Announcement channel is now <#{}>", &channel_id)).await?;
        } else {
            msg.channel_id.say(&ctx.http, "Error while setting announcement channel.").await?;
        }

    } else {
        msg.channel_id.say(&ctx.http, "You have to provide proper a text channel mention or name or id! Ughh whatever").await?;
    }

    Ok(())
}