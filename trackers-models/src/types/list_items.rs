#[derive(
    Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature = "diesel", diesel(sql_type = crate::db::schema::sql_types::ListItemT))]
pub struct ListItem {
    pub item_content: String,
    #[serde(rename = "checkmarked")]
    pub is_completed: bool,
}

#[cfg(feature = "diesel")]
impl diesel::serialize::ToSql<crate::db::schema::sql_types::ListItemT, diesel::pg::Pg>
    for ListItem
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        diesel::serialize::WriteTuple::<(diesel::sql_types::Text, diesel::sql_types::Bool)>::write_tuple(
            &(&self.item_content, self.is_completed),
            &mut out.reborrow(),
        )
    }
}

#[cfg(feature = "diesel")]
impl diesel::deserialize::FromSql<crate::db::schema::sql_types::ListItemT, diesel::pg::Pg>
    for ListItem
{
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let val = <(String, bool) as diesel::deserialize::FromSql<
            diesel::sql_types::Record<(diesel::sql_types::Text, diesel::sql_types::Bool)>,
            diesel::pg::Pg,
        >>::from_sql(bytes)?;
        Ok(ListItem {
            item_content: val.0,
            is_completed: val.1,
        })
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature = "diesel", diesel(sql_type=diesel::sql_types::Array<diesel::sql_types::Nullable<crate::db::schema::sql_types::ListItemT>>))]
#[cfg_attr(feature = "diesel", diesel(sql_type=diesel::sql_types::Array<crate::db::schema::sql_types::ListItemT>))]
pub struct ListItems(pub Vec<ListItem>);

impl From<Vec<ListItem>> for ListItems {
    fn from(value: Vec<ListItem>) -> Self {
        Self(value)
    }
}

impl From<ListItems> for Vec<ListItem> {
    fn from(value: ListItems) -> Self {
        value.0
    }
}

#[cfg(feature = "diesel")]
impl
    diesel::serialize::ToSql<
        diesel::sql_types::Array<
            diesel::sql_types::Nullable<crate::db::schema::sql_types::ListItemT>,
        >,
        diesel::pg::Pg,
    > for ListItems
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        diesel::serialize::ToSql::<
            diesel::sql_types::Array<
                diesel::sql_types::Nullable<crate::db::schema::sql_types::ListItemT>,
            >,
            _,
        >::to_sql(&self.0, out)
    }
}

#[cfg(feature = "diesel")]
impl
    diesel::deserialize::FromSql<
        diesel::sql_types::Array<
            diesel::sql_types::Nullable<crate::db::schema::sql_types::ListItemT>,
        >,
        diesel::pg::Pg,
    > for ListItems
{
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Self(Vec::<ListItem>::from_sql(bytes)?))
    }
}
