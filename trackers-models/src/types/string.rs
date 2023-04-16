/// Length limited String type based on [std::string::String].
///
/// Besides the length limit, the type also asserts that no strings with empty
/// contents are created. The goal behind that is to avoid storing confusing
/// data in the database. If the intention is to have no value for a given
/// field or column then setting it to NULL or equivalent is preferred.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature="diesel", diesel(sql_type=diesel::sql_types::Text))]
pub struct String<const MAX_LEN: usize>(std::string::String);

impl<const L: usize> std::fmt::Display for String<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum StringLengthError<const MAX_LEN: usize> {
    TooLong,
    EmptyString,
}

impl<const L: usize> std::fmt::Display for StringLengthError<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringLengthError::TooLong => write!(
                f,
                "provided string is too long, expected maximum length of {L} bytes"
            ),
            StringLengthError::EmptyString => write!(f, "cannot accept a string with no content"),
        }
    }
}
impl<const L: usize> std::error::Error for StringLengthError<L> {}

impl<const L: usize> TryFrom<std::string::String> for String<L> {
    type Error = StringLengthError<L>;

    fn try_from(value: std::string::String) -> Result<Self, Self::Error> {
        if value.len() > L {
            Err(StringLengthError::TooLong)
        } else if value.len() == 0 {
            Err(StringLengthError::EmptyString)
        } else {
            Ok(Self(value))
        }
    }
}

impl<const L: usize> schemars::JsonSchema for String<L> {
    fn schema_name() -> std::string::String {
        "length-limited string".to_owned()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::InstanceType;
        use schemars::schema::StringValidation;

        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            metadata: Some(Box::new(schemars::schema::Metadata {
                title: Some(std::string::String::from("length-limited string")),
                description: Some(format!("A length-limited UTF-8 string which length (definded as size) can be up to __{L} bytes__. There is also an additional constrain on the type specifying that it cannot be an empty, zero-length string.")),
                default: None,
                examples: Vec::new(),
                ..schemars::schema::Metadata::default()
            })),
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                InstanceType::String,
            ))),
            string: Some(Box::new(StringValidation { max_length: Some(L as u32), min_length: Some(1), pattern: None })), //TODO
            ..schemars::schema::SchemaObject::default()
        })
    }
}

impl<const L: usize> serde::Serialize for String<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de, const L: usize> serde::Deserialize<'de> for String<L> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringVisitor<const L: usize>;

        impl<'de, const L: usize> serde::de::Visitor<'de> for StringVisitor<L> {
            type Value = String<L>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a non-empty string with length up to {L} bytes")
            }
            fn visit_string<E>(self, v: std::string::String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let s: String<L> = v
                    .try_into()
                    .map_err(|e: StringLengthError<L>| serde::de::Error::custom(e.to_string()))?;
                Ok(s)
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let s: String<L> = v
                    .to_owned()
                    .try_into()
                    .map_err(|e: StringLengthError<L>| serde::de::Error::custom(e.to_string()))?;
                Ok(s)
            }
        }

        deserializer.deserialize_string(StringVisitor::<L>)
    }
}

#[cfg(feature = "diesel")]
impl<const L: usize> diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg>
    for String<L>
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <std::string::String as diesel::serialize::ToSql<diesel::sql_types::Text, _>>::to_sql(
            &self.0, out,
        )
    }
}

#[cfg(feature = "diesel")]
impl<const L: usize> diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg>
    for String<L>
{
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let s = <std::string::String as diesel::deserialize::FromSql<
            diesel::sql_types::Text,
            _,
        >>::from_sql(bytes)?;
        Ok(s.try_into()?)
    }
}
