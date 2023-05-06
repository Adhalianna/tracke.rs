#[derive(Debug, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct UserCreation {
    pub email: crate::types::Email,
    pub password: crate::types::PasswordInput,
}
