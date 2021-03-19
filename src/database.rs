use serenity::prelude::TypeMapKey;

use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
    Pool,
    Postgres
};

pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool { 
    type Value = PgPool;
}

pub async fn connect(uri: &String) 
-> Result<Pool<Postgres>, Box<dyn std::error::Error + Send + Sync>> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(uri)
        .await?;

    Ok(pool)
}