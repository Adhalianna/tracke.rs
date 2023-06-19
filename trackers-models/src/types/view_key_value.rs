use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature="diesel", diesel(sql_type = crate::db::schema::sql_types::ViewKvT))]
/// Key and value making up one of query parameters that define a view over a given tracker
pub struct ViewKV {
    /// Key of a query parameter used to request the view
    pub key: crate::types::String<64>,
    /// Value for the given key used as a query parameter
    pub value: crate::types::String<64>,
}

#[cfg(feature = "diesel")]
impl diesel::serialize::ToSql<crate::db::schema::sql_types::ViewKvT, diesel::pg::Pg> for ViewKV {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        diesel::serialize::WriteTuple::<(diesel::sql_types::Text, diesel::sql_types::Text)>::write_tuple(
            &(&self.key, &self.value),
            &mut out.reborrow(),
        )
    }
}

#[cfg(feature = "diesel")]
impl diesel::deserialize::FromSql<crate::db::schema::sql_types::ViewKvT, diesel::pg::Pg>
    for ViewKV
{
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let val = <(String, String) as diesel::deserialize::FromSql<
            diesel::sql_types::Record<(diesel::sql_types::Text, diesel::sql_types::Text)>,
            diesel::pg::Pg,
        >>::from_sql(bytes)?;
        Ok(ViewKV {
            key: crate::types::String::try_from(val.0)
                .expect("the length of string for key stored in the database should not exceed 64"),
            value: crate::types::String::try_from(val.1).expect(
                "the length of string stored for value in the database should not exceed 64",
            ),
        })
    }
}

impl schemars::JsonSchema for ViewKV {
    fn schema_name() -> String {
        "view key-value".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{
            InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec, StringValidation,
        };

        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
            object: Some(Box::new(ObjectValidation {
                max_properties: Some(1),
                min_properties: Some(1),
                required: BTreeSet::new(),
                properties: BTreeMap::new(),
                pattern_properties: {
                    let mut map = BTreeMap::new();
                    map.insert(
                        ".*".to_owned(),
                        Schema::Object(SchemaObject {
                            instance_type: Some(SingleOrVec::Single(Box::new(
                                InstanceType::String,
                            ))),
                            string: Some(Box::new(StringValidation {
                                max_length: None,
                                min_length: None,
                                pattern: Some(".*".to_owned()),
                            })),
                            ..Default::default()
                        }),
                    );
                    map
                },
                ..Default::default()
            })),

            ..Default::default()
        })
    }
}

impl serde::Serialize for ViewKV {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut ser = serializer.serialize_map(Some(1))?;
        ser.serialize_entry(&self.key, &self.value)?;
        ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ViewKV {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ViewKVVisitor;
        impl<'de> serde::de::Visitor<'de> for ViewKVVisitor {
            type Value = ViewKV;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a single mapping from a key to a value")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let (key, value) = map.next_entry()?.ok_or(serde::de::Error::custom(
                    "expected a single pair of key and value",
                ))?;
                Ok(ViewKV { key, value })
            }
        }

        deserializer.deserialize_map(ViewKVVisitor)
    }
}
