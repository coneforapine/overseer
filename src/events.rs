use serenity::{
    model::{
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
    async_trait
};

use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is now watching!", ready.user.name);
    }
}

