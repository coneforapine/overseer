pub mod queries;
pub mod models;

use serenity::prelude::TypeMapKey;
use tracing::info;

use sqlx::{
    postgres::PgPoolOptions,
    Postgres,
    PgPool,
    Pool
};
use color_eyre::eyre::Result;

pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

pub async fn connect(uri: &String)
                     -> Result<Pool<Postgres>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Database uri from the connect: {:?}", uri);
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(uri)
        .await?;

    Ok(pool)
}