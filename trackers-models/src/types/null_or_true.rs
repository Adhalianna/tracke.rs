/// Maps all false or missing values into [`Null`](NullOrTrue::Null) variant.
///
/// In PostgreSQL each `NULL` value is by default considered as distinct from
/// any other `NULL`. This property can be useful in creating some constraints
/// and expressing interesting relations on the table level. The application
/// uses this in the context of boolean typed columns.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
    Default,
)]
#[serde(from = "Option<bool>")]
#[serde(into = "Option<bool>")]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature="diesel", diesel(sql_type=diesel::sql_types::Nullable<diesel::sql_types::Bool>))]
pub enum NullOrTrue {
    /// `NULL` that would be stored in the database
    #[default]
    Null,
    True,
}

impl From<Option<bool>> for NullOrTrue {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(b) => match b {
                true => NullOrTrue::True,
                false => NullOrTrue::Null,
            },
            None => NullOrTrue::Null,
        }
    }
}

impl From<&Option<bool>> for NullOrTrue {
    fn from(value: &Option<bool>) -> Self {
        match value {
            Some(b) => match b {
                true => NullOrTrue::True,
                false => NullOrTrue::Null,
            },
            None => NullOrTrue::Null,
        }
    }
}

impl Into<Option<bool>> for NullOrTrue {
    fn into(self) -> Option<bool> {
        match self {
            NullOrTrue::Null => None,
            NullOrTrue::True => Some(true),
        }
    }
}

impl<'a> Into<&'a Option<bool>> for &'a NullOrTrue {
    fn into(self) -> &'a Option<bool> {
        match self {
            NullOrTrue::Null => &None,
            NullOrTrue::True => &Some(true),
        }
    }
}

impl From<bool> for NullOrTrue {
    fn from(value: bool) -> Self {
        match value {
            true => NullOrTrue::True,
            false => NullOrTrue::Null,
        }
    }
}

#[cfg(feature = "diesel")]
impl diesel::serialize::ToSql<diesel::sql_types::Nullable<diesel::sql_types::Bool>, diesel::pg::Pg>
    for NullOrTrue
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <Option<bool> as diesel::serialize::ToSql<
            diesel::sql_types::Nullable<diesel::sql_types::Bool>,
            _,
        >>::to_sql(self.into(), out)
    }
}

#[cfg(feature = "diesel")]
impl
    diesel::deserialize::FromSql<
        diesel::sql_types::Nullable<diesel::sql_types::Bool>,
        diesel::pg::Pg,
    > for NullOrTrue
{
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let opt = <Option<bool> as diesel::deserialize::FromSql<
            diesel::sql_types::Nullable<diesel::sql_types::Bool>,
            _,
        >>::from_sql(bytes)?;
        Ok(NullOrTrue::from(opt))
    }
}
