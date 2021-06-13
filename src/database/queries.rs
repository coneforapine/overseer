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

    pub async fn insert_new(user_id: String, mod_id: String, case_type: Option<CaseType>, reason: Option<String>, pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {

        if let Some(re) = reason {
            sqlx::query!("\
                INSERT INTO cases(user_id, moderator_id, case_type, reason)\
                    VALUES($1, $2, $3, $4);
            ",
            user_id, mod_id, case_type as Option<CaseType>, re
            ).execute(pool).await
        } else {
            sqlx::query!(
            "INSERT INTO cases(user_id, moderator_id, case_type) VALUES ($1, $2, $3);",
            user_id, mod_id, case_type as Option<CaseType>
            ).execute(pool).await
        }
    }

}