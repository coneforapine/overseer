#[derive(sqlx::FromRow)]
pub struct GuildSettings {
    pub id: i32,
    pub guild_id: String,
    pub announcement_channel: Option<String>
}