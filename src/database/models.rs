use sqlx::types::Uuid;

#[derive(Debug)]
pub struct GuildSettings {
    pub id: Uuid,
    pub guild_id: String,
    pub announcement_channel: Option<String>,
    pub case_channel: Option<String>
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "case_types", rename_all = "lowercase")]
pub enum CaseType {
    Ban,
    Warn,
    Kick
}

#[derive(Debug)]
pub struct Case {
    pub id: Uuid,
    pub user_id: String,
    pub moderator_id: String,
    pub case_type: CaseType,
    pub number: i32,
    pub reason: Option<String>
}
