#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
#[cfg_attr(feature = "diesel", derive(diesel::Queryable, diesel::Insertable))]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::trackers))]
pub struct Tracker {
    pub tracker_id: crate::types::Uuid,
    pub user_id: crate::types::Uuid,
    pub name: crate::types::String<256>,
    /// Informs whether the tracker is marked as _default_. A default tracker
    /// cannot be deleted and all tasks with no `tracker_id` assigned will be
    /// assigned to that default tracker.
    #[serde(default, skip_deserializing)]
    pub is_default: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct TrackerInput {
    #[serde(default)]
    pub tracker_id: Option<crate::types::Uuid>,
    #[serde(default)]
    pub user_id: Option<crate::types::Uuid>,
    pub name: crate::types::String<256>,
}
