#![allow(unstable_name_collisions)]

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PasswordInput(std::string::String);

impl std::fmt::Display for PasswordInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl PasswordInput {
    /// # Panics
    /// Expects the `self` to be hashable with bcrypt.
    pub fn into_storeable(self) -> Vec<u8> {
        let hashed: String = bcrypt::hash(self.0, bcrypt::DEFAULT_COST)
            .expect("failed to convert password input into storeable password");
        hashed.into_bytes()
    }
    /// # Panics
    /// Expects that the stored bytes can be decoded as UTF-8 string and
    /// expectes the `self` to be hashable with bcrypt.
    pub fn match_with(&self, stored: Vec<u8>) -> bool {
        let stored: String =
            String::from_utf8(stored).expect("failed to read stored password bytes");
        bcrypt::verify(&self.0, &stored).unwrap()
    }
}

#[derive(Clone, Hash, Debug)]
pub enum InvalidPassword {
    /// A password should have at least 8 characters
    TooShort,
    /// Characters from one or more character classes are missing
    MissingCharcter(Vec<RequieredCharacters>),
    /// The provided character cannot be used in a password
    BadCharacter(char),
}

impl std::fmt::Display for InvalidPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;

        match self {
            InvalidPassword::TooShort => write!(
                f,
                "provided password is too short, required at least 8 characters"
            ),
            InvalidPassword::MissingCharcter(missed) => 
            write!(
                f,
                "missed characters from the following required character classes: {}",
                missed
                    .into_iter()
                    .map(ToString::to_string)
                    .intersperse("; ".to_string())
                    .collect::<String>()
            ),
            InvalidPassword::BadCharacter(bad) => {
                write!(f, "provided character {bad} cannot be used in a password")
            }
        }
    }
}

impl std::error::Error for InvalidPassword {}

#[derive(Clone, Hash, Debug)]
pub enum RequieredCharacters {
    SpecialCharacter, // Character set: !?#$%^&*@-+=
    Letter,
    Digit,
}

impl std::fmt::Display for RequieredCharacters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequieredCharacters::SpecialCharacter => {
                write!(f, "character from a set expressed as string: !?#$%^&@-+=")
            }
            RequieredCharacters::Letter => write!(f, "an alphabetic unicode character"),
            RequieredCharacters::Digit => write!(f, "a numeric unicode character"),
        }
    }
}

impl TryFrom<String> for PasswordInput {
    type Error = InvalidPassword;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // check length
        if value.len() < 8 {
            return Err(InvalidPassword::TooShort);
        }

        // check if required characters are present
        let mut has_letter = false;
        let mut has_digit = false;
        let mut has_special_char = false;
        value.chars().for_each(|ch| {
            if ch.is_alphabetic() {
                has_letter = true
            }
            if ch.is_numeric() {
                has_digit = true
            }
            if ['!', '?', '#', '$', '%', '^', '&', '@', '-', '+', '=']
                .iter()
                .find(|c| **c == ch)
                .is_some()
            {
                has_special_char = true
            }
        });
        if !(has_letter && has_digit && has_special_char) {
            return Err(InvalidPassword::MissingCharcter({
                let mut missed_requirements: Vec<RequieredCharacters> = Vec::new();
                if !has_letter {
                    missed_requirements.push(RequieredCharacters::Letter);
                }
                if !has_digit {
                    missed_requirements.push(RequieredCharacters::Letter);
                }
                if !has_special_char {
                    missed_requirements.push(RequieredCharacters::SpecialCharacter);
                }
                missed_requirements
            }));
        }

        // make sure there are no funky characters
        if let Some(bad_char) = value.chars().into_iter().find_map(|ch| {
            if ch.is_ascii_control() {
                Some(InvalidPassword::BadCharacter(ch))
            } else {
                None
            }
        }) {
            return Err(bad_char);
        }

        // everything is fine
        return Ok(Self(value));
    }
}

impl schemars::JsonSchema for PasswordInput {
    fn schema_name() -> std::string::String {
        "password string".to_owned()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::InstanceType;

        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            metadata: Some(Box::new(schemars::schema::Metadata {
                title: Some(std::string::String::from("password string")),
                description: Some(String::from(
                    r#"A password to be accepted must meet the following criteria:
- it must be at least _8_ characters long
- it must have at least _one letter_ which is an alphabetic unicode character
- it must have at least _one digit_ which is a numeric unicode character
- it must have at least one character from the following set __`!?#$%^&*@-+=`__
- it must not contain control characters"#,
                )),
                default: None,
                examples: vec![
                    "my$26Password#15".into(),
                    "i|nl<pwxFWU&!kr".into(),
                    "+i]wbX!\\1??b;A!>Zk!zF2[dD=".into(),
                ],
                ..schemars::schema::Metadata::default()
            })),
            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                InstanceType::String,
            ))),
            ..schemars::schema::SchemaObject::default()
        })
    }
}

impl<'de> serde::Deserialize<'de> for PasswordInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PasswordVisitor;

        impl<'de> serde::de::Visitor<'de> for PasswordVisitor {
            type Value = PasswordInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "password string")
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let pass: PasswordInput = v
                    .try_into()
                    .map_err(|e: InvalidPassword| serde::de::Error::custom(e.to_string()))?;
                Ok(pass)
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let pass: PasswordInput = v
                    .to_string()
                    .try_into()
                    .map_err(|e: InvalidPassword| serde::de::Error::custom(e.to_string()))?;
                Ok(pass)
            }
        }

        deserializer.deserialize_string(PasswordVisitor)
    }
}
