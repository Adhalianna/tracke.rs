#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(transparent)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)
)]
#[cfg_attr(feature="diesel", diesel(sql_type=diesel::sql_types::VarChar))]
pub struct ClientSecretStr(String);

impl ClientSecretStr {
    /// Generates a cryptographically random string of length of at least 64 characters
    pub fn new() -> Self {
        use std::io::Write;

        let mut buf: [u8; 64] = [0; 64];
        let write_res: Result<Vec<_>, _> = buf
            .chunks_exact_mut(32)
            .map(|mut sli| sli.write(&userspace_rng::random256()).and(Ok(sli)))
            .collect();
        write_res.expect("the writing to buffer while generating the client secret string was expected to always succeed");

        let mut str_buf = String::with_capacity(64);
        str_buf = buf.chunks_exact(16).fold(str_buf, |str_buf, sli| {
            str_buf + &base62::encode(u128::from_ne_bytes(sli.try_into().unwrap()))
        });

        Self(str_buf)
    }
}

/// We want to have just `Into<String>` and no `From<String>` to avoid creating
/// accidentially a secret from a non-random string that is not
/// cryptographically safe.
impl Into<String> for ClientSecretStr {
    fn into(self) -> String {
        self.0
    }
}

impl std::fmt::Display for ClientSecretStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::VarChar, diesel::pg::Pg> for ClientSecretStr {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <String as diesel::serialize::ToSql<diesel::sql_types::VarChar, _>>::to_sql(&self.0, out)
    }
}
impl diesel::deserialize::FromSql<diesel::sql_types::VarChar, diesel::pg::Pg> for ClientSecretStr {
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::pg::Pg>,
    ) -> diesel::deserialize::Result<Self> {
        let str =
            <String as diesel::deserialize::FromSql<diesel::sql_types::VarChar, _>>::from_sql(
                bytes,
            )?;
        Ok(Self(str))
    }
}

impl schemars::JsonSchema for ClientSecretStr {
    fn schema_name() -> String {
        "client secret string".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::SingleOrVec;
        use schemars::schema::{InstanceType, Metadata};

        let schema_obj = schemars::schema::SchemaObject {
            metadata: Some(Box::new(Metadata {
                id: None,
                title: Some("client secret string".to_owned()),
                description: Some(String::from("A string of random characters that is used eiter as a `client_id` for OAuth2 Client Credentials flow or as a `client_secret` in the same flow. Its length is variable and has bounds <64, 128>.")),
                default: None,
                deprecated: false,
                read_only: true,
                write_only: false,
                examples: Vec::new(),
            })),
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            .. Default::default()
        };

        schemars::schema::Schema::Object(schema_obj)
    }
}
