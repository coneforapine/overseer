use sqlx::types::Uuid;

#[derive(Debug)]
pub struct GuildSettings {
    pub id: Uuid,
    pub guild_id: String,
    pub announcement_channel: Option<String>,
    pub case_channel: Option<String>
}