#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature="diesel",diesel(sql_type=diesel::sql_types::Timestamp))]
pub struct DeadlineTime(chrono::NaiveDateTime);

#[cfg(feature = "diesel")]
impl diesel::serialize::ToSql<diesel::sql_types::Timestamp, diesel::pg::Pg> for DeadlineTime {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <chrono::NaiveDateTime as diesel::serialize::ToSql<diesel::sql_types::Timestamp, _>>::to_sql(
            &self.0, out,
        )
    }
}

#[cfg(feature = "diesel")]
impl diesel::deserialize::FromSql<diesel::sql_types::BigInt, diesel::pg::Pg> for DeadlineTime {
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let time = <chrono::NaiveDateTime as diesel::deserialize::FromSql<
            diesel::sql_types::Timestamp,
            _,
        >>::from_sql(bytes)?;
        Ok(Self(time.into()))
    }
}

impl serde::Serialize for DeadlineTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
