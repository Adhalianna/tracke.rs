#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug, Default)]
#[serde(transparent)]
#[cfg_attr(feature = "diesel", derive(diesel::expression::AsExpression))]
#[cfg_attr(feature="diesel", diesel(sql_type=diesel::sql_types::Array<diesel::sql_types::Nullable<diesel::sql_types::Text>>))]
pub struct Tags(pub Vec<String>);

impl ToString for &Tags {
    fn to_string(&self) -> String {
        self.0.join(", ")
    }
}

#[cfg(feature = "diesel")]
impl
    diesel::serialize::ToSql<
        diesel::sql_types::Array<diesel::sql_types::Nullable<diesel::sql_types::Text>>,
        diesel::pg::Pg,
    > for Tags
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        use diesel::serialize::ToSql;

        <Vec<String> as ToSql<
            diesel::sql_types::Array<diesel::sql_types::Nullable<diesel::sql_types::Text>>,
            diesel::pg::Pg,
        >>::to_sql(&self.0, out)
    }
}

#[cfg(feature = "diesel")]
impl
    diesel::deserialize::FromSql<
        diesel::sql_types::Array<diesel::sql_types::Nullable<diesel::sql_types::Text>>,
        diesel::pg::Pg,
    > for Tags
{
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Self(Vec::<String>::from_sql(bytes)?))
    }
}

