/// A request for `client_id` and `client_secret` that can be used by an
/// external client application to access the API
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
pub struct ClientCredentialsRequest {
    /// ID of requesting user
    pub user_id: crate::types::Uuid,
    /// Name of the client assigned by the user
    pub name: String,
    /// An URL at which more information can be found about the client
    /// appliaction
    pub website: String,
}

impl ClientCredentialsRequest {
    pub fn to_authorised_client(self) -> AuthorisedClientFull {
        AuthorisedClientFull {
            user_id: self.user_id,
            name: self.name,
            website: self.website,
            client_id: crate::types::ClientSecretStr::new(),
            client_secret: crate::types::ClientSecretStr::new(),
        }
    }
}

/// Description of a client application that has been authorised by a user to
/// act in their name using `client_id` and `client_secret` instead of username
/// and password. The `client_secret` is only available in
/// [`AuthorisedClientFull`]. Removing the `client_secret` from this struct is
/// meant to protect API users from accidentially exposing the secret when in
/// it was not required.
/// Just like [`AuthorisedClient`] but with the secret included
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::Queryable, diesel::Associations, diesel::AsChangeset)
)]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::authorised_clients))]
#[cfg_attr(
    feature = "diesel",
    diesel(belongs_to(crate::db::user::User, foreign_key = user_id))
)]
pub struct AuthorisedClient {
    /// ID of requesting user    
    pub user_id: crate::types::Uuid,
    /// Name of the client assigned by the user    
    pub name: String,
    /// An URL at which more information can be found about the client
    /// appliaction
    pub website: String,
    /// The id given to the client application that can be used for OAuth2
    /// Client Credentials flow
    pub client_id: crate::types::ClientSecretStr,
}

/// Just like [`AuthorisedClient`] but with the secret included
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, schemars::JsonSchema)]
#[cfg_attr(
    feature = "diesel",
    derive(
        diesel::Queryable,
        diesel::Insertable,
        diesel::Associations,
        diesel::AsChangeset
    )
)]
#[cfg_attr(feature="diesel", diesel(table_name = crate::db::schema::authorised_clients))]
#[cfg_attr(
    feature = "diesel",
    diesel(belongs_to(crate::db::user::User, foreign_key = user_id))
)]
pub struct AuthorisedClientFull {
    /// ID of requesting user    
    pub user_id: crate::types::Uuid,
    /// Name of the client assigned by the user    
    pub name: String,
    /// An URL at which more information can be found about the client
    /// appliaction
    pub website: String,
    /// The ID given to the client application that can be used for OAuth2
    /// Client Credentials flow
    pub client_id: crate::types::ClientSecretStr,
    /// The secret assigned to the client application neccessary
    pub client_secret: crate::types::ClientSecretStr,
}

impl From<AuthorisedClientFull> for AuthorisedClient {
    fn from(value: AuthorisedClientFull) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name,
            website: value.website,
            client_id: value.client_id,
        }
    }
}
