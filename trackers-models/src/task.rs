use super::Duration;

#[derive(
    diesel::Queryable,
    diesel::Insertable,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    Clone,
)]
#[diesel(table_name = crate::db::schema::tasks)]
pub struct Task {
    pub task_id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub group_id: Option<uuid::Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub time_estimate: Option<Duration>,
    pub soft_deadline: Option<chrono::NaiveDateTime>,
    pub hard_deadline: Option<chrono::NaiveDateTime>,
    pub tags: Option<Vec<Option<String>>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
pub struct TaskInput {
    pub task_id: Option<uuid::Uuid>,
    pub group_id: Option<uuid::Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub time_estimate: Option<Duration>,
    pub soft_deadline: Option<chrono::NaiveDateTime>,
    pub hard_deadline: Option<chrono::NaiveDateTime>,
    pub tags: Option<Vec<Option<String>>>,
}
