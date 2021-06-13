use dotenv::dotenv;
use serde::Deserialize;
use color_eyre::{eyre::WrapErr, Result};
use tracing_subscriber::EnvFilter;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub prefix: String,
    pub database_url: String
}

impl BotConfig {
    // Basically loads .env file and returns it as BotConfig
    #[instrument]
    pub fn from_env() -> Result<BotConfig> {
        dotenv().ok();
        
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading bot configuration from .env file");

        let mut conf = config::Config::new();
        conf.merge(config::Environment::default())?;
        
        conf.try_into().context("Error while loading .env")
    }

}
