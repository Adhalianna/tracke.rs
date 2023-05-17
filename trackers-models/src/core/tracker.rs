#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
#[cfg_attr(
    feature = "diesel",
    derive(
        diesel::Queryable,
        diesel::Insertable,
        diesel::Associations,
        diesel::AsChangeset
    )
)]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::trackers))]
#[cfg_attr(
    feature = "diesel",
    diesel(belongs_to(crate::db::user::User, foreign_key = user_id))
)]
pub struct Tracker {
    pub tracker_id: crate::types::Uuid,
    pub user_id: crate::types::Uuid,
    pub name: crate::types::String<256>,
    /// Informs whether the tracker is marked as _default_. A default tracker
    /// cannot be deleted and all tasks with no `tracker_id` assigned will be
    /// assigned to that default tracker.
    #[serde(default, skip_deserializing)]
    pub is_default: crate::types::NullOrTrue,
}

/// The default tracker that should be created as the first tracker for every new user.
impl Default for Tracker {
    fn default() -> Self {
        Self {
            tracker_id: crate::types::Uuid::new(),
            user_id: crate::types::Uuid::new(),
            name: "Default Task Tracker".to_owned().try_into().unwrap(),
            is_default: true.into(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
#[cfg_attr(feature = "diesel", derive(diesel::AsChangeset))]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::trackers, treat_none_as_null = false))]
pub struct TrackerReplace {
    pub tracker_id: crate::types::Uuid,
    pub user_id: crate::types::Uuid,
    pub name: crate::types::String<256>,
    /// The `Option` layered on top of the [`NullOrTrue`](crate::types::NullOrTrue)
    /// with the `default` and `skip_serializing ` are important to obtain the
    /// expected value for the [diesel::AsChangeset] implementation
    #[serde(default, skip_deserializing)]
    pub is_default: Option<crate::types::NullOrTrue>,
}

impl From<Tracker> for TrackerReplace {
    fn from(value: Tracker) -> Self {
        Self {
            tracker_id: value.tracker_id,
            user_id: value.user_id,
            name: value.name,
            is_default: None,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
#[cfg_attr(feature = "diesel", derive(diesel::AsChangeset))]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::trackers, treat_none_as_null = false))]
pub struct TrackerPatch {
    #[serde(default)]
    pub tracker_id: Option<crate::types::Uuid>,
    #[serde(default)]
    pub user_id: Option<crate::types::Uuid>,
    #[serde(default)]
    pub name: Option<crate::types::String<256>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct TrackerInput {
    #[serde(default)]
    pub tracker_id: Option<crate::types::Uuid>,
    #[serde(default)]
    pub user_id: Option<crate::types::Uuid>,
    pub name: crate::types::String<256>,
}
