#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct View {
    pub view_id: crate::types::Uuid,
    pub user_id: crate::types::Uuid,
    pub name: crate::types::String<256>,
    pub trackers: Vec<TrackerView>,
}

/// Input values used to create a new view over multiple trackers. As a result
/// of view creation a `view_id` should be assigned which allows creating the
/// proper `View`
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct CreateView {
    pub user_id: crate::types::Uuid,
    pub name: crate::types::String<256>,
    pub trackers: Vec<TrackerView>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct TrackerView {
    pub tracker_id: crate::types::Uuid,
    pub tracker_name: crate::types::String<256>,
    pub name: Option<crate::types::String<256>>,
    pub keys_values: Vec<crate::types::ViewKV>,
}
