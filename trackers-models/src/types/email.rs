#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature="diesel", diesel(sql_type=diesel::sql_types::VarChar))]
pub struct Email(std::string::String);

impl TryFrom<String> for Email {
    type Error = fast_chemail::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        fast_chemail::parse_email(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for Email {
    type Error = fast_chemail::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fast_chemail::parse_email(&value)?;
        Ok(Self(value.to_owned()))
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl schemars::JsonSchema for Email {
    fn schema_name() -> std::string::String {
        "email address".to_owned()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::InstanceType;
        use schemars::schema::StringValidation;

        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            metadata: Some(Box::new(schemars::schema::Metadata {
                title: Some(std::string::String::from("email address")),
                description: Some(String::from(
                    "An email address compliant with the [HTML specification](https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address) of a valid address.",
                )),
                examples: vec![
                    "john.doe@example.com".into(),
                    "jane.smith+symbol@example.net".into(),
                ],
                ..schemars::schema::Metadata::default()
            })),
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                InstanceType::String,
            ))),
            string: Some(Box::new(StringValidation {
                max_length: None,
                min_length: None,
                pattern: Some(String::from(r#"^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"#)),
            })),
            ..schemars::schema::SchemaObject::default()
        })
    }
}

impl serde::Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmailVisitor;

        impl<'de> serde::de::Visitor<'de> for EmailVisitor {
            type Value = Email;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an email address")
            }
            fn visit_string<E>(self, v: std::string::String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.try_into().map_err(|e: fast_chemail::ParseError| {
                    serde::de::Error::custom(e.to_string())
                })?)
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.try_into().map_err(|e: fast_chemail::ParseError| {
                    serde::de::Error::custom(e.to_string())
                })?)
            }
        }

        deserializer.deserialize_string(EmailVisitor)
    }
}

#[cfg(feature = "diesel")]
impl diesel::serialize::ToSql<diesel::sql_types::VarChar, diesel::pg::Pg> for Email {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <std::string::String as diesel::serialize::ToSql<diesel::sql_types::VarChar, _>>::to_sql(
            &self.0, out,
        )
    }
}

#[cfg(feature = "diesel")]
impl diesel::deserialize::FromSql<diesel::sql_types::VarChar, diesel::pg::Pg> for Email {
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let v = <std::string::String as diesel::deserialize::FromSql<
            diesel::sql_types::VarChar,
            _,
        >>::from_sql(bytes)?;
        Ok(v.try_into()?)
    }
}
