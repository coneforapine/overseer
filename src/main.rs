mod botconfig;
mod database;
mod commands;
mod utils;

use tracing::{info, error};
use serenity::{Client, framework::StandardFramework, http::Http};

use botconfig::BotConfig;
use commands::ADMIN_GROUP;

#[tokio::main]
async fn main() {

    let conf = BotConfig::from_env().expect("Botconfig");
    let http = Http::new_with_token(&conf.token);
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
                Err(why) => panic!("Could not access bot id: {:?}", why)
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why)
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix(&conf.prefix)
            .owners(owners)
        ).group(&ADMIN_GROUP);

    let mut client = Client::builder(&conf.token)
        .framework(framework)
        .await
        .expect("Error creating the client");

    // Adding database pool to client data.
    info!("{:?}", &conf.database_url);
    let pool = database::connect(&conf.database_url).await.unwrap();

    {
        let mut data = client.data.write().await;
        data.insert::<database::ConnectionPool>(pool);
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}