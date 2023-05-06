/// The request is issued when a user starts account creation process.
/// To complete the process the user should use the confirmation code that
/// should be generated and sent to the selected email address. When the
/// the request is confirmed with the code it will be sent back to the user
/// once more with updated `confirmed_with_code` field. After that the request
/// resource will be removed from the database and the user will be able to log
/// into his newly created account using the password sent during the first
/// action in the registration process.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct RegistrationRequest {
    /// The timestamp of the request.
    pub issued_at: chrono::DateTime<chrono::offset::Utc>,
    /// How long can the request be confirmed with the confirmation code until
    /// it becomes invalid.
    pub valid_until: chrono::DateTime<chrono::offset::Utc>,
    /// The email that will be used to create the account.
    pub user_email: crate::types::Email,
    /// Echo back to the user that he has accepted the ToS by sending the
    /// registration request.
    pub accepted_tos: bool,
    /// Has the request been confirmed with the confirmation code that should
    /// be sent to the selected email address?
    pub confirmed_with_code: bool,
}

// NOTE: Expected links after resource creation:
// - confirm (link to where the confirmation code should be posted)
// - cancel (link to self)
// - code (echo once more the email)
// After confirmation the resource should be returned from the API with
// `confirmed_with_code` set to `true`.

impl From<crate::db::RegistrationRequest> for RegistrationRequest {
    fn from(value: crate::db::RegistrationRequest) -> Self {
        Self {
            issued_at: value.issued_at,
            valid_until: value.valid_until,
            user_email: value.email,
            accepted_tos: true,
            confirmed_with_code: false,
        }
    }
}
