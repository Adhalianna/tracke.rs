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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature = "diesel", diesel(sql_type=diesel::sql_types::Array<diesel::sql_types::Nullable<crate::db::schema::sql_types::ListItemT>>))]
#[cfg_attr(feature = "diesel", diesel(sql_type=diesel::sql_types::Array<crate::db::schema::sql_types::ListItemT>))]
pub struct ListItems(pub Vec<ListItem>);

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
struct ListItemSer {
    pub idx: usize,
    pub item_content: String,
    pub checkmarked: bool,
}

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

impl serde::Serialize for ListItems {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq_ser = serializer.serialize_seq(Some(self.0.len()))?;
        let res: Result<Vec<_>, S::Error> = self
            .0
            .iter()
            .enumerate()
            .map(|(idx, item)| (idx + 1, item))
            .map(|(idx, item)| {
                seq_ser.serialize_element(&ListItemSer {
                    idx,
                    checkmarked: item.is_completed,
                    item_content: item.item_content.clone(),
                })
            })
            .collect();
        res?;
        seq_ser.end()
    }
}

struct ItemsListVisitor;

impl<'de> serde::de::Visitor<'de> for ItemsListVisitor {
    type Value = ListItems;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an array of list items")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut tmp_store = Vec::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some(item) = seq.next_element::<ListItemSer>()? {
            tmp_store.push((
                item.idx,
                ListItem {
                    item_content: item.item_content,
                    is_completed: item.checkmarked,
                },
            ));
        }

        tmp_store.sort_by(|(idx1, _), (idx2, _)| idx1.cmp(idx2));

        Ok(ListItems(
            tmp_store.into_iter().map(|(_, item)| item).collect(),
        ))
    }
}

impl<'de> serde::Deserialize<'de> for ListItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(ItemsListVisitor)
    }
}

impl schemars::JsonSchema for ListItems {
    fn schema_name() -> String {
        String::from("list items")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<ListItemSer>()
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
