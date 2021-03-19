mod botconfig;
mod commands;
mod database;
mod events;
mod utils;
mod models;

use tracing::error;
use serenity::{Client, framework::StandardFramework, http::Http};

use botconfig::BotConfig;
use events::Handler;

use commands::TEST_GROUP;
use commands::ADMIN_GROUP;

#[tokio::main]
async fn main() {
    
    let conf = BotConfig::from_env().expect("Bot configuration");
    let token = conf.token;
    let prefix = conf.prefix;
    let db_uri = conf.database_url;

    let http = Http::new_with_token(&token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = std::collections::HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match  http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("could not access bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why)
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix(&prefix)
            .owners(owners))
            .group(&TEST_GROUP)
            .group(&ADMIN_GROUP);
    
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let pool = database::connect(&db_uri).await.unwrap();

    {
        let mut data = client.data.write().await;
        data.insert::<database::ConnectionPool>(pool);
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
    
}