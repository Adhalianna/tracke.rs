#[derive(Debug, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct UserCreation {
    pub email: crate::types::Email,
    pub password: crate::types::PasswordInput,
    /// Explicit acceptance of the Terms of Service is required to create an account
    pub accepted_tos: bool,
}
