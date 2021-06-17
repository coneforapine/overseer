use serenity::{
    model::prelude::*,
    prelude::*,
    utils::{parse_channel},
};
use crate::database::models::CaseType;

// A more detailed channel parsing function
// Priority of parsing:
// 1. Mention
// 2. Channel ID
// 3. Channel name
// 4. Part of a channel name
pub async fn parse_chan(
    name: &str,
    optional_gid: Option<&GuildId>,
    optional_ctx: Option<&Context>,
) -> Option<ChannelId> {
    if let Some(x) = parse_channel(&name) {
        return Some(ChannelId(x));
    }

    let gid = match optional_gid {
        Some(g) => g,
        None => return None,
    };

    let ctx = match optional_ctx {
        Some(c) => c,
        None => return None,
    };

    if let Ok(id) = name.parse::<u64>() {
        if let Some(x) = ChannelId(id).to_channel_cached(&ctx).await {
            return Some(x.id());
        }
    }

    let guild = match gid.to_guild_cached(&ctx).await {
        Some(g) => g,
        None => return None,
    };

    for (key, value) in guild.channels.iter() {
        let cname = &value.name;
        if cname == name {
            return Some(*key);
        }
    }

    for (key, value) in guild.channels.iter() {
        let cname = &value.name;
        if cname.contains(name) {
            return Some(*key);
        }
    }

    None
}

pub fn match_case_enum(case_type: CaseType) -> String {
    match case_type {
        CaseType::Warn => String::from("Warning"),
        CaseType::Ban => String::from("Ban"),
        CaseType::Kick => String::from("Kick")
    }
}