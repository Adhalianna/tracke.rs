/// Task is the primiary unit of information wihtin the `tracke.rs` application.
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    diesel::Queryable,
    diesel::Insertable,
    diesel::Associations,
)]
#[diesel(table_name = crate::db::schema::tasks)]
#[diesel(belongs_to(crate::core::Tracker, foreign_key = tracker_id))]
pub struct Task {
    /// Universally unique task identifier
    pub task_id: crate::types::Uuid,
    /// The ID of a tracker to which the task belongs
    pub tracker_id: crate::types::Uuid,
    pub completed_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The title of a task. As it is the only required descriptive field it can also be
    /// used as the sole description of the task.
    pub title: crate::types::String<256>,
    /// Optional longer description of the task.
    pub description: Option<crate::types::String<4096>>,
    pub time_estimate: Option<crate::types::Duration>,
    pub soft_deadline: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub hard_deadline: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub tags: Option<crate::types::Tags>,
}

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    diesel::Queryable,
    diesel::Insertable,
    diesel::Associations,
    diesel::AsChangeset,
)]
#[diesel(table_name = crate::db::schema::tasks)]
#[diesel(belongs_to(crate::core::Tracker, foreign_key = tracker_id))]
pub struct TaskPatch {
    #[serde(default)]
    pub task_id: Option<crate::types::Uuid>,
    #[serde(default)]
    pub tracker_id: Option<crate::types::Uuid>,
    #[serde(default)]
    pub title: Option<crate::types::String<256>>,
    #[serde(default)]
    pub completed_at: Option<Option<chrono::DateTime<chrono::offset::Utc>>>,
    #[serde(default)]
    pub description: Option<Option<crate::types::String<4096>>>,
    #[serde(default)]
    pub time_estimate: Option<Option<crate::types::Duration>>,
    #[serde(default)]
    pub soft_deadline: Option<Option<chrono::DateTime<chrono::offset::Utc>>>,
    #[serde(default)]
    pub hard_deadline: Option<Option<chrono::DateTime<chrono::offset::Utc>>>,
    #[serde(default)]
    pub tags: Option<Option<crate::types::Tags>>,
}

impl From<crate::core::TaskPatch> for TaskPatch {
    fn from(t: crate::core::TaskPatch) -> Self {
        Self {
            task_id: t.task_id,
            tracker_id: t.tracker_id,
            completed_at: {
                match (t.checkmarked, t.checkmarked_at) {
                    (None, None) => None,
                    (None, Some(c_at)) => Some(c_at),
                    (Some(check), None) => match check {
                        true => Some(Some(chrono::Utc::now())),
                        false => Some(None),
                    },
                    (Some(check), Some(c_at)) => match (check, c_at) {
                        (true, None) => Some(Some(chrono::Utc::now())),
                        (true, Some(c_at)) => Some(Some(c_at)),
                        (false, None) => Some(None),
                        (false, Some(_)) => Some(None),
                    },
                }
            },
            title: t.title,
            description: t.description,
            time_estimate: t.time_estimate,
            soft_deadline: t.soft_deadline,
            hard_deadline: t.hard_deadline,
            tags: t.tags,
        }
    }
}
