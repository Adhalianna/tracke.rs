#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, diesel::Queryable, diesel::Insertable,
)]
#[diesel(table_name = crate::db::schema::registration_requests)]
pub struct RegistrationRequest {
    pub email: crate::types::Email,
    pub issued_at: chrono::DateTime<chrono::offset::Utc>,
    pub valid_until: chrono::DateTime<chrono::offset::Utc>,
    pub confirmation_code: String,
    pub password: Vec<u8>,
}
