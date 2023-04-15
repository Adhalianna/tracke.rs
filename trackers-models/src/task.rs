use super::Duration;
use super::Tags;

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
#[cfg_attr(feature = "diesel", derive(diesel::Queryable, diesel::Insertable))]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::tasks))]
pub struct Task {
    pub task_id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub group_id: Option<uuid::Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub time_estimate: Option<Duration>,
    pub soft_deadline: Option<chrono::NaiveDateTime>,
    pub hard_deadline: Option<chrono::NaiveDateTime>,
    pub tags: Option<Tags>,
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
    pub tags: Option<Tags>,
}
