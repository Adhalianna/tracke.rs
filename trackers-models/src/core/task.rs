/// Task is the primiary unit of information wihtin the `tracke.rs` application.
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
pub struct Task {
    /// Universally unique task identifier
    pub task_id: uuid::Uuid,
    /// The ID of a tracker to which the task belongs
    pub tracker_id: uuid::Uuid,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub completed_at: Option<chrono::NaiveDateTime>,
    /// The title of a task. As it is the only required descriptive field it can
    /// also be used as the sole description of the task.
    pub title: crate::types::String<256>,
    /// Optional longer description of the task.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<crate::types::String<4096>>,
    /// The estimated amount of time it will take to complete the task.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time_estimate: Option<crate::types::Duration>,
    /// A self-imposed deadline. Missing this kind of deadline does not imply
    /// failing the task.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub soft_deadline: Option<chrono::NaiveDateTime>,
    /// A hard deadline is such a deadline which if missed it implies that the
    /// task can no longer be completed.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hard_deadline: Option<chrono::NaiveDateTime>,
    /// Tags can be used to make filtering the tasks simple even between
    /// different task trackers.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<crate::types::Tags>,
}

#[cfg(feature = "diesel")]
impl From<crate::db::Task> for Task {
    fn from(t: crate::db::Task) -> Self {
        Self {
            task_id: t.task_id,
            tracker_id: t.tracker_id,
            completed: t.completed_at.is_some(),
            completed_at: t.completed_at,
            title: t.title,
            description: t.description,
            time_estimate: t.time_estimate,
            soft_deadline: t.soft_deadline,
            hard_deadline: t.hard_deadline,
            tags: t.tags,
        }
    }
}

/// Input values for the [Task] model.
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
pub struct TaskInput {
    pub task_id: Option<uuid::Uuid>,
    pub tracker_id: uuid::Uuid,
    pub title: crate::types::String<256>,
    #[serde(default)]
    pub completed: bool,
    #[serde(default)]
    pub completed_at: Option<chrono::NaiveDate>,
    #[serde(default)]
    pub description: Option<crate::types::String<4096>>,
    #[serde(default)]
    pub time_estimate: Option<crate::types::Duration>,
    #[serde(default)]
    pub soft_deadline: Option<chrono::NaiveDateTime>,
    #[serde(default)]
    pub hard_deadline: Option<chrono::NaiveDateTime>,
    #[serde(default)]
    pub tags: Option<crate::types::Tags>,
}
