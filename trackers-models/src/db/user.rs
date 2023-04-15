#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, diesel::Queryable, diesel::Insertable,
)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub password: Vec<u8>,
}
