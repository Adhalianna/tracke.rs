/// A request for `client_id` and `client_secret` that can be used by an
/// external client application to access the API
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct ClientCredentialsRequest {
    /// ID of requesting user
    pub user_id: crate::types::Uuid,
    /// Name of the client assigned by the user
    pub name: String,
    /// A name of the website at which more information can be found about the
    /// client
    pub website: String,
}

/// Description of a client application that has been authorised by a user to
/// act in their name using `client_id` and `client_secret` instead of username
/// and password. The `client_secret` is only available in
/// [`AuthorisedClientFull`]. Removing the `client_secret` from this struct is
/// meant to protect API users from accidentially exposing the secret when in
/// it was not required.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct AuthorisedClient {
    /// ID of requesting user    
    pub user_id: crate::types::Uuid,
    pub name: String,
    pub website: String,
    pub client_id: String,
}

/// Just like [`AuthorisedClient`] but with the secret included
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct AuthorisedClientFull {
    /// ID of requesting user    
    pub user_id: crate::types::Uuid,
    pub name: String,
    pub website: String,
    pub client_id: String,
    pub client_secret: String,
}
