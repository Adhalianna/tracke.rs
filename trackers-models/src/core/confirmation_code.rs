use std::fmt::Display;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[serde(rename = "confirmation_code")]
#[serde(transparent)]
pub struct ConfirmationCode(std::string::String);

impl ConfirmationCode {
    pub fn new() -> Self {
        Self(random_string::generate(
            9,
            "1234567890ABCDEFGHIJKLMNOPRSTUWXYZ@#$%&",
        ))
    }
}

impl Into<String> for ConfirmationCode {
    fn into(self) -> String {
        self.0
    }
}

impl PartialEq<String> for ConfirmationCode {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl Display for ConfirmationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl schemars::JsonSchema for ConfirmationCode {
    fn schema_name() -> String {
        "registration confirmation code".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::InstanceType;
        use schemars::schema::StringValidation;

        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            metadata: Some(Box::new(schemars::schema::Metadata {
                title: Some(std::string::String::from("registration confirmation code")),
                description: Some(String::from(
                    r#"A 9-character long code used to confirm registration request. Expected to be sent in a body of application/json type."#,
                )),
                default: None,
                examples: vec!["1H7Z&O9PL".into()],
                ..schemars::schema::Metadata::default()
            })),
            string: Some(Box::new(StringValidation {
                max_length: Some(9),
                min_length: Some(9),
                pattern: Some(String::from("[1234567890ABCDEFGHIJKLMNOPRSTUWXYZ@#$%&]{9}")),
            })),
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                InstanceType::String,
            ))),
            ..schemars::schema::SchemaObject::default()
        })
    }
}
