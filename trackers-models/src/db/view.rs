use std::collections::HashMap;

use crate::db::schema::tracker_views;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Hash,
    PartialEq,
    Eq,
    diesel::Queryable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Associations,
)]
#[diesel(table_name = crate::db::schema::views)]
#[
    diesel(belongs_to(crate::db::user::User, foreign_key = user_id))
]
pub struct View {
    pub view_id: crate::types::Uuid,
    pub user_id: crate::types::Uuid,
    pub name: crate::types::String<256>,
}

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Hash,
    diesel::Queryable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Associations,
)]
#[diesel(table_name = crate::db::schema::tracker_views)]
#[diesel(belongs_to(crate::db::schema::views::table, foreign_key = view_id))]
pub struct TrackerView {
    pub view_id: crate::types::Uuid,
    pub tracker_id: crate::types::Uuid,
    pub name: Option<crate::types::String<256>>,
    pub keys_values: ViewKVs,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    diesel::deserialize::FromSqlRow,
    diesel::expression::AsExpression,
    serde::Serialize,
    serde::Deserialize,
)]
#[diesel(sql_type=diesel::sql_types::Array<diesel::sql_types::Nullable<crate::db::schema::sql_types::ViewKvT>>)]
#[diesel(sql_type=diesel::sql_types::Array<crate::db::schema::sql_types::ViewKvT>)]
pub struct ViewKVs(pub Vec<crate::types::ViewKV>);

impl
    diesel::serialize::ToSql<
        diesel::sql_types::Array<
            diesel::sql_types::Nullable<crate::db::schema::sql_types::ViewKvT>,
        >,
        diesel::pg::Pg,
    > for ViewKVs
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        diesel::serialize::ToSql::<
            diesel::sql_types::Array<
                diesel::sql_types::Nullable<crate::db::schema::sql_types::ViewKvT>,
            >,
            _,
        >::to_sql(&self.0, out)
    }
}

impl
    diesel::deserialize::FromSql<
        diesel::sql_types::Array<
            diesel::sql_types::Nullable<crate::db::schema::sql_types::ViewKvT>,
        >,
        diesel::pg::Pg,
    > for ViewKVs
{
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Self(Vec::<crate::types::ViewKV>::from_sql(bytes)?))
    }
}

pub struct TmpViewVec(pub Vec<crate::core::View>);

impl From<Vec<(View, TrackerView, String)>> for TmpViewVec {
    fn from(value: Vec<(View, TrackerView, String)>) -> Self {
        // If we observed that the results are always returned ordered in such a way that first we get
        // all records for one view then all for another and so on, then maybe a smarter idea would be
        // to create the desired vector right away. This however is very early and rushed developement
        // so we will not test if that's the case and use an intermediary hashmap instead.

        let mut map =
        // random untested capacity value, trying to prealloc some sane defaults
            HashMap::<View, Vec<(TrackerView, String)>>::with_capacity((value.len() / 4).max(4));

        use std::collections::hash_map::Entry;

        for record in value {
            let (view, tracker_view, tracker_name) = record;

            match map.entry(view) {
                Entry::Occupied(mut entry) => {
                    let stored = entry.get_mut();
                    stored.push((tracker_view, tracker_name));
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![(tracker_view, tracker_name)]);
                }
            }
        }

        let mut result = Vec::with_capacity(map.len());

        for view in map {
            let (view, tracker_views) = view;
            result.push(crate::core::View {
                view_id: view.view_id,
                user_id: view.user_id,
                name: view.name,
                trackers: tracker_views
                    .into_iter()
                    .map(|v| crate::core::TrackerView {
                        tracker_id: v.0.tracker_id,
                        tracker_name: v.1.try_into().unwrap(),
                        keys_values: v.0.keys_values.0,
                        name: v.0.name,
                    })
                    .collect(),
            })
        }

        TmpViewVec(result)
    }
}

impl Into<Vec<crate::core::View>> for TmpViewVec {
    fn into(self) -> Vec<crate::core::View> {
        self.0
    }
}

impl Into<(crate::db::View, Vec<crate::db::TrackerView>)> for crate::core::View {
    fn into(self) -> (crate::db::View, Vec<crate::db::TrackerView>) {
        (
            crate::db::View {
                view_id: self.view_id.clone(),
                user_id: self.user_id,
                name: self.name,
            },
            self.trackers
                .into_iter()
                .map(|v| TrackerView {
                    view_id: self.view_id.clone(),
                    tracker_id: v.tracker_id,
                    name: v.name,
                    keys_values: ViewKVs(v.keys_values),
                })
                .collect(),
        )
    }
}
