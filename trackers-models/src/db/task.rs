/// Task is the primiary unit of information wihtin the `tracke.rs` application.
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, diesel::Queryable, diesel::Insertable,
)]
#[diesel(table_name = crate::db::schema::tasks)]
pub struct Task {
    /// Universally unique task identifier
    pub task_id: uuid::Uuid,
    /// The ID of a tracker to which the task belongs
    pub tracker_id: uuid::Uuid,
    pub completed_at: Option<chrono::NaiveDateTime>,
    /// The title of a task. As it is the only required descriptive field it can also be
    /// used as the sole description of the task.
    pub title: crate::types::String<256>,
    /// Optional longer description of the task.
    pub description: Option<crate::types::String<4096>>,
    pub time_estimate: Option<crate::types::Duration>,
    pub soft_deadline: Option<chrono::NaiveDateTime>,
    pub hard_deadline: Option<chrono::NaiveDateTime>,
    pub tags: Option<crate::types::Tags>,
}
