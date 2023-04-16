#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "diesel", derive(diesel::Queryable, diesel::Insertable))]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::trackers))]
pub struct Tracker {
    pub tracker_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: crate::types::String<256>,
    /// Informs whether the tracker is marked as _default_. A default tracker
    /// cannot be deleted and all tasks with no `tracker_id` assigned will be
    /// assigned to that default tracker.
    #[serde(default, skip_deserializing)]
    pub is_default: bool,
}
