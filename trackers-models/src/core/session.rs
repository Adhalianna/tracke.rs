#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
#[cfg_attr(feature = "diesel", derive(diesel::Queryable, diesel::Insertable))]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::sessions))]
pub struct Session {
    pub user_id: crate::types::Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub started_at: chrono::DateTime<chrono::offset::Utc>,
    pub valid_until: chrono::DateTime<chrono::offset::Utc>,
}
