use crate::prelude::*;
use axum::async_trait;
pub use scope::{PathAndScope, VariableScope};
pub mod layer;
pub mod scope;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserClaims {
    exp: aliri_clock::UnixTime,
    iss: aliri::jwt::Issuer,
    aud: aliri::jwt::Audiences,
    jti: uuid::Uuid,
    scope: aliri_oauth2::oauth2::Scope,
}

impl aliri::jwt::CoreClaims for UserClaims {
    fn nbf(&self) -> Option<aliri_clock::UnixTime> {
        None
    }
    fn exp(&self) -> Option<aliri_clock::UnixTime> {
        Some(self.exp)
    }
    fn aud(&self) -> &aliri::jwt::Audiences {
        &self.aud
    }
    fn iss(&self) -> Option<&aliri::jwt::IssuerRef> {
        Some(&self.iss)
    }
    fn sub(&self) -> Option<&aliri::jwt::SubjectRef> {
        None
    }
}

impl aliri_oauth2::oauth2::HasScope for UserClaims {
    fn scope(&self) -> &aliri_oauth2::Scope {
        &self.scope
    }
}

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for UserClaims
where
    S: Send + Sync,
{
    type Rejection = aliri_axum::AuthFailed;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Some(claims): Option<UserClaims> = parts.extensions.remove() else {
            return Err(aliri_axum::AuthFailed::MissingClaims)
        };
        Ok(claims)
    }
}
