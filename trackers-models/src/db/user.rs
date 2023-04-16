#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, diesel::Queryable, diesel::Insertable,
)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub user_id: crate::types::Uuid,
    pub email: crate::types::Email,
    pub password: Vec<u8>,
}
