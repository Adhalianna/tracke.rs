/// An uuid which is presented in a base62 encoded form when serialized with
/// serde but stored as a normal uuid in the database.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature="diesel", diesel(sql_type=diesel::sql_types::Uuid))]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", base62::encode(self.0.as_u128()))
    }
}

impl std::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            base62::encode(self.0.as_u128()),
            self.0.hyphenated()
        )
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl schemars::JsonSchema for Uuid {
    fn schema_name() -> std::string::String {
        "base62-encoded uuid".to_owned()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::InstanceType;
        use schemars::schema::StringValidation;

        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            metadata: Some(Box::new(schemars::schema::Metadata {
                title: Some(std::string::String::from("base62-encoded uuid")),
                description: Some(String::from("An uuid encoded with [base62](https://en.wikipedia.org/wiki/Base62) algorithm to achieve a shorter representation that fits well in URLs.")),
                default: None,
                examples: vec![Uuid(uuid::Uuid::now_v7()).to_string().into(), Uuid(uuid::Uuid::default()).to_string().into(), "5wbwf6yUxVBcr48AMbz9cb".into()],
                ..schemars::schema::Metadata::default()
            })),
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                InstanceType::String,
            ))),
            string: Some(Box::new(StringValidation {
                max_length: Some(22),
                min_length: None,
                pattern: Some(String::from("[0-9A-Za-z]")),
            })),
            ..schemars::schema::SchemaObject::default()
        })
    }
}

impl serde::Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(base62::encode(self.0.as_u128()).as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UuidVisitor;

        impl<'de> serde::de::Visitor<'de> for UuidVisitor {
            type Value = Uuid;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a base62 encoded UUID value")
            }
            fn visit_string<E>(self, v: std::string::String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let num = base62::decode(v).map_err(|e| serde::de::Error::custom(e.to_string()))?;
                let uuid = uuid::Uuid::from_u128(num);
                Ok(Uuid(uuid))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let num = base62::decode(v).map_err(|e| serde::de::Error::custom(e.to_string()))?;
                let uuid = uuid::Uuid::from_u128(num);
                Ok(Uuid(uuid))
            }
        }

        deserializer.deserialize_str(UuidVisitor)
    }
}

#[cfg(feature = "diesel")]
impl diesel::serialize::ToSql<diesel::sql_types::Uuid, diesel::pg::Pg> for Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <uuid::Uuid as diesel::serialize::ToSql<diesel::sql_types::Uuid, _>>::to_sql(&self.0, out)
    }
}

#[cfg(feature = "diesel")]
impl diesel::deserialize::FromSql<diesel::sql_types::Uuid, diesel::pg::Pg> for Uuid {
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let v = <uuid::Uuid as diesel::deserialize::FromSql<diesel::sql_types::Uuid, _>>::from_sql(
            bytes,
        )?;
        Ok(Self(v))
    }
}
