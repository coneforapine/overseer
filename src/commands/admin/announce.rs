use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
        Args
    },
    model::channel::Message,
    prelude::Context,
};

use tracing::{info, warn};
use eyre::anyhow;

use crate::database::ConnectionPool;
use crate::utils::parse_chan;
use crate::models::GuildSettings;

#[command]
#[sub_commands(set_channel)]
#[only_in(guilds)]
#[min_args(1)]
#[usage("announce <message>")]
pub async fn announce(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Guild Id not found."))?;

    let guild_settings: Option<GuildSettings> = sqlx::query_as!(
        GuildSettings,
        "SELECT * FROM guild_settings WHERE guild_id=$1",
        guild_id.to_string()
    )
    .fetch_optional(pool)
    .await?;

    
    if let Some(gs) = guild_settings {
        if let Some(ac) = gs.announcement_channel {
            let ac = ac.parse::<u64>().unwrap();

            let c = guild_id.channels(&ctx.http).await?;
            let c = c.values().find(|c| c.id == ac);

            if let Some(channel) = c {
                channel.send_message(&ctx.http, |message|
                    message.content(&args.message())
                ).await?;
            } else {
                msg.reply(&ctx.http, "Announcement channel might be deleted or inaccesable.").await?;
            }

        } else {
            msg.reply(&ctx.http, "Dude you didn't set announcement channel.").await?;
        }
    } else {
        warn!("empty guild settings");
    }

    
    Ok(())
}

#[command("set")]
#[only_in(guilds)]
#[min_args(1)]
#[max_args(1)]
#[usage("announce set <channel: guild channel>")]
pub async fn set_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;
    if let Some(channel_id) = parse_chan(
        &args.quoted()
            .current()
            .ok_or_else(|| anyhow!("Channel not found"))?
            .to_string(),
            Some(&guild_id),
            Some(&ctx)
    ).await {
        info!("{:?}", msg.content);
        
        info!("Trying to insert guild_settings table with params: {:?}, {:?}", guild_id, channel_id);    
        sqlx::query!("
            INSERT INTO guild_settings (guild_id, announcement_channel) values ($1, $2)
                ON CONFLICT (guild_id) DO
                    UPDATE SET announcement_channel = $2
            ",
            &guild_id.to_string(),
            &channel_id.to_string())
        .execute(pool)
        .await?;
        msg.reply(&ctx.http, format!("Announcement channel set as <#{}>", &channel_id)).await?;
    }

    Ok(())
}
