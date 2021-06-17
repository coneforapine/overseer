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

use tracing::debug;



#[command]
#[usage("case")]
pub async fn case(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    if args.is_empty() {
        msg.channel_id.say(&ctx, "And the Pinecone said let there be case number").await?;
    }

    let case_number = args.current().unwrap().parse::<i32>().unwrap();

    if let Some(case) = Case::get_from_case_number(case_number, pool).await? {
        let acted_user = UserId(case.user_id.parse::<u64>().unwrap()).to_user(&ctx.http).await?;
        let moderator = UserId(case.moderator_id.parse::<u64>().unwrap()).to_user(&ctx.http).await?;

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("The case of {}#{}", acted_user.name, acted_user.discriminator))
                    .field("Reason", case.reason.unwrap(), true)
                    .field("Moderator", format!("<!@{}>", moderator.id) , true)
                    .field("Action type", match_case_enum(case.case_type), true)
            })
        }).await?;
    } else {
        msg.channel_id.say(&ctx.http, "There is no case with that number. Are u sure?").await?
    }

    Ok(())
}