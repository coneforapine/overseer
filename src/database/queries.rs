use super::models::{GuildSettings, Case};

use sqlx::{Pool, Postgres, Error};
use sqlx::postgres::PgQueryResult;
use crate::database::models::CaseType;

impl GuildSettings {

    pub async fn fetch_one(guild_id: String, pool: &Pool<Postgres>) -> Option<GuildSettings> {
        let guild_settings = sqlx::query_as!(GuildSettings,
            "SELECT * FROM guild_settings WHERE guild_id=$1;",
            guild_id
        ).fetch_optional(pool).await.ok()?;

        guild_settings
    }

    pub async fn insert_new(guild_id: &String, pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        sqlx::query!(
            "INSERT INTO guild_settings(guild_id) VALUES ($1) ON CONFLICT DO NOTHING;",
            guild_id
        ).execute(pool).await
    }

    pub async fn update_single_field(guild_id: String, field: String, value: String, pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        let sql = format!("\
            UPDATE guild_settings \
            SET {} = '{}' \
            WHERE guild_id = '{}';",
            field, value, guild_id
        );

        sqlx::query(&sql).execute(pool).await
    }
}

impl Case {
    // Returns case number
    pub async fn insert_new(user_id: String, mod_id: String, case_type: CaseType, reason: String, pool: &Pool<Postgres>) -> anyhow::Result<i32> {
        let rec = sqlx::query!(r#"
            INSERT INTO cases(user_id, moderator_id, reason, case_type)
                VALUES($1, $2, $3, $4)
                 RETURNING number;
        "#,
        user_id, mod_id, reason, case_type as CaseType
        ).fetch_one(pool).await?;

        Ok(rec.number)

    }
}