use schemars::schema::InstanceType;

/// Stored in the database as a number of seconds
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature="diesel",diesel(sql_type=diesel::sql_types::BigInt))]
pub struct Duration(pub chrono::Duration);

impl AsRef<chrono::Duration> for Duration {
    fn as_ref(&self) -> &chrono::Duration {
        &self.0
    }
}

#[cfg(feature = "diesel")]
impl diesel::serialize::ToSql<diesel::sql_types::BigInt, diesel::pg::Pg> for Duration {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let duration = self.0.num_seconds();
        <i64 as diesel::serialize::ToSql<diesel::sql_types::BigInt, _>>::to_sql(
            &duration,
            &mut out.reborrow(),
        )?;
        Ok(diesel::serialize::IsNull::No)
    }
}

#[cfg(feature = "diesel")]
impl diesel::deserialize::FromSql<diesel::sql_types::BigInt, diesel::pg::Pg> for Duration {
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let i64_value = <i64 as diesel::deserialize::FromSql<
            diesel::sql_types::BigInt,
            diesel::pg::Pg,
        >>::from_sql(bytes)?;
        Ok(Self(chrono::Duration::seconds(i64_value).into()))
    }
}

impl schemars::JsonSchema for Duration {
    fn schema_name() -> String {
        "duration".to_owned()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                metadata: Some(Box::new(schemars::schema::Metadata {
                    title: Some(String::from("duration")),
                    description: Some(String::from("The duration string is a concatenation of time spans. Where each time span is an integer number and a suffix. Supported suffixes:\n
* __seconds__, __second__, __sec__, __s__\n
* __minutes__, __minute__, __min__, __m__\n
* __hours__, __hour__, __hr__, __h__\n
* __days__, __day__, __d__\n
* __weeks__, __week__, __w__\n
* __months__, __month__, __M__ – defined as 30.44 days\n
* __years__, __year__, __y__ – defined as 365.25 days\n")),
                    default: None,  //TODO
                    examples: vec![
                        serde_json::Value::String("5m 30s".to_string()), 
                        serde_json::Value::String("1w 2d".to_string()), 
                        serde_json::Value::String("120m 30s".to_string())
                    ],
                    ..schemars::schema::Metadata::default()
                })),
                instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(InstanceType::String))),
                format: None,        //TODO
                string: None,        //TODO
                ..schemars::schema::SchemaObject::default()
            })
    }
}

impl serde::Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            &humantime::format_duration(
                self.0
                    .to_std()
                    .map_err(|e| serde::ser::Error::custom(e.to_string()))?,
            )
            .to_string(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DurationVisitor;
        impl<'de> serde::de::Visitor<'de> for DurationVisitor {
            type Value = chrono::Duration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a human readable duration string")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                humantime::parse_duration(v)
                    .map(|d| {
                        chrono::Duration::from_std(d)
                            .map_err(|e| serde::de::Error::custom(e.to_string()))
                    })
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?
            }
        }

        Ok(Self(deserializer.deserialize_string(DurationVisitor)?))
    }
}
