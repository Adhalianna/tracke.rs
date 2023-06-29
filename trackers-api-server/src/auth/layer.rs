use std::{str::FromStr, sync::OnceLock};

use super::UserClaims;
use axum::response::IntoResponse;

const ISSUER: OnceLock<String> = OnceLock::new();
const AUDIENCE: OnceLock<String> = OnceLock::new();
const KEY_ID: OnceLock<String> = OnceLock::new();
const SHARED_SECRET: OnceLock<String> = OnceLock::new();

pub async fn require_jwt<B: axum::body::HttpBody>(
    request: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> axum::response::Response {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        axum::http::header::WWW_AUTHENTICATE,
        axum::http::HeaderValue::from_str("Bearer realm=\"tracke.rs\" ")
            .expect("failed to save JWT into response headers"),
    );

    response
}

pub fn validator() -> &'static aliri::jwt::CoreValidator {
    static VALID: once_cell::sync::OnceCell<aliri::jwt::CoreValidator> =
        once_cell::sync::OnceCell::new();

    VALID.get_or_init(|| {
        aliri::jwt::CoreValidator::default()
            .add_approved_algorithm(aliri::jwa::Algorithm::HS256)
            .add_allowed_audience(
                aliri::jwt::Audience::from_str(AUDIENCE.get_or_init(|| {
                    #[cfg(feature = "local-dev")]
                    let audience = dotenvy::var("JWT_AUDIENCE")
                        .expect("JWT_AUDIENCE environment variable must be set in the .env file");
                    #[cfg(not(feature = "local-dev"))]
                    let audience = std::env::var("JWT_AUDIENCE")
                        .expect("JWT_AUDIENCE environment variable must be set");
                    audience
                }))
                .unwrap(),
            )
            .require_issuer(
                aliri::jwt::Issuer::from_str(ISSUER.get_or_init(|| {
                    #[cfg(feature = "local-dev")]
                    let issuer = dotenvy::var("JWT_ISSUER")
                        .expect("JWT_ISSUER environment variable must be set in the .env file");
                    #[cfg(not(feature = "local-dev"))]
                    let issuer = std::env::var("JWT_ISSUER")
                        .expect("JWT_ISSUER environment variable must be set");
                    issuer
                }))
                .unwrap(),
            )
    })
}

pub fn authority() -> &'static aliri_oauth2::Authority {
    static AUTH: once_cell::sync::OnceCell<aliri_oauth2::Authority> =
        once_cell::sync::OnceCell::new();
    AUTH.get_or_init(|| {
        let key = aliri::Jwk::from(aliri::jwa::Hmac::new(
            SHARED_SECRET
                .get_or_init(|| {
                    #[cfg(feature = "local-dev")]
                    let secret = dotenvy::var("JWT_SHARED_SECRET").expect(
                        "JWT_SHARED_SECRET environment variable must be set in the .env file",
                    );
                    #[cfg(not(feature = "local-dev"))]
                    let secret = std::env::var("JWT_SHARED_SECRET")
                        .expect("JWT_SHARED_SECRET environment variable must be set");
                    secret
                })
                .to_owned()
                .into_bytes(),
        ))
        .with_algorithm(aliri::jwa::Algorithm::HS256)
        .with_key_id(
            aliri::jwk::KeyId::from_str(KEY_ID.get_or_init(|| {
                #[cfg(feature = "local-dev")]
                let key = dotenvy::var("JWT_KEY_ID")
                    .expect("JWT_KEY_ID environment variable must be set in the .env file");
                #[cfg(not(feature = "local-dev"))]
                let key = std::env::var("JWT_KEY_ID")
                    .expect("JWT_KEY_ID environment variable must be set");
                key
            }))
            .unwrap(),
        );

        let mut jwks = aliri::Jwks::default();
        jwks.add_key(key);

        aliri_oauth2::Authority::new(jwks, validator().clone())
    })
}

#[derive(Clone, Default)]
pub struct AuthErrorHandler;

impl aliri_tower::OnJwtError for AuthErrorHandler {
    type Body = axum::body::BoxBody;

    fn on_missing_or_malformed(&self) -> axum::http::Response<Self::Body> {
        crate::error::UnathorizedError::default()
            .with_msg("authorization token is missing or malformed")
            .with_docs()
            .with_links([("new session", "/api/session/token".into())])
            .into_response()
    }

    fn on_no_matching_jwk(&self) -> axum::http::Response<Self::Body> {
        crate::error::UnathorizedError::default()
            .with_msg("failed to find matching JWK")
            .with_docs()
            .with_links([("new session", "/api/session/token".into())])
            .into_response()
    }

    fn on_jwt_invalid(
        &self,
        error: aliri::error::JwtVerifyError,
    ) -> axum::http::Response<Self::Body> {
        crate::error::UnathorizedError::default()
            .with_msg(error)
            .with_docs()
            .with_links([("new session", "/api/session/token".into())])
            .into_response()
    }
}

impl aliri_tower::OnScopeError for AuthErrorHandler {
    type Body = axum::body::BoxBody;

    fn on_missing_scope_claim(&self) -> axum::http::Response<Self::Body> {
        crate::error::UnathorizedError::default()
            .with_msg("authorization token is missing scope claims")
            .with_docs()
            .with_links([("new session", "/api/session/token".into())])
            .into_response()
    }

    fn on_scope_policy_failure(
        &self,
        _held: &aliri_oauth2::Scope,
        _policy: &aliri_oauth2::ScopePolicy,
    ) -> axum::http::Response<Self::Body> {
        crate::error::UnathorizedError::default()
            .with_msg(format!("failed to meet required policies with held token"))
            .with_links([("new session", "/api/session/token".into())])
            .with_docs()
            .into_response()
    }
}

pub fn authorizer() -> aliri_tower::Oauth2Authorizer<crate::auth::UserClaims, AuthErrorHandler> {
    aliri_tower::Oauth2Authorizer::new()
        .with_claims::<crate::auth::UserClaims>()
        .with_error_handler(AuthErrorHandler::default())
}

pub fn new_token_with_exp_and_scopes(
    lifetime_in_seconds: u64,
    scopes: aliri_oauth2::Scope,
) -> aliri::Jwt {
    let jwt_headers = aliri::jwt::BasicHeaders::with_key_id(
        aliri::jwa::Algorithm::HS256,
        aliri::jwk::KeyId::from_str(KEY_ID.get_or_init(|| {
            #[cfg(feature = "local-dev")]
            let key = dotenvy::var("JWT_KEY_ID")
                .expect("JWT_KEY_ID environment variable must be set in the .env file");
            #[cfg(not(feature = "local-dev"))]
            let key =
                std::env::var("JWT_KEY_ID").expect("JWT_KEY_ID environment variable must be set");
            key
        }))
        .unwrap(),
    );
    let jwk = aliri::Jwk::from(aliri::jwa::Hmac::new(
        SHARED_SECRET
            .get_or_init(|| {
                #[cfg(feature = "local-dev")]
                let secret = dotenvy::var("JWT_SHARED_SECRET")
                    .expect("JWT_SHARED_SECRET environment variable must be set in the .env file");
                #[cfg(not(feature = "local-dev"))]
                let secret = std::env::var("JWT_SHARED_SECRET")
                    .expect("JWT_SHARED_SECRET environment variable must be set");
                secret
            })
            .to_owned()
            .into_bytes(),
    ))
    .with_algorithm(aliri::jwa::Algorithm::HS256)
    .with_key_id(
        aliri::jwk::KeyId::from_str(KEY_ID.get_or_init(|| {
            #[cfg(feature = "local-dev")]
            let key = dotenvy::var("JWT_KEY_ID")
                .expect("JWT_KEY_ID environment variable must be set in the .env file");
            #[cfg(not(feature = "local-dev"))]
            let key =
                std::env::var("JWT_KEY_ID").expect("JWT_KEY_ID environment variable must be set");
            key
        }))
        .unwrap(),
    );
    let claims = UserClaims {
        exp: aliri_clock::UnixTime::from({
            std::time::SystemTime::now()
                .checked_add(std::time::Duration::from_secs(lifetime_in_seconds))
                .unwrap()
        }),
        iss: aliri::jwt::Issuer::new(
            ISSUER
                .get_or_init(|| {
                    #[cfg(feature = "local-dev")]
                    let issuer = dotenvy::var("JWT_ISSUER")
                        .expect("JWT_ISSUER environment variable must be set in the .env file");
                    #[cfg(not(feature = "local-dev"))]
                    let issuer = std::env::var("JWT_ISSUER")
                        .expect("JWT_ISSUER environment variable must be set");
                    issuer
                })
                .to_owned(),
        ),
        aud: aliri::jwt::Audience::new(
            AUDIENCE
                .get_or_init(|| {
                    #[cfg(feature = "local-dev")]
                    let audience = dotenvy::var("JWT_AUDIENCE")
                        .expect("JWT_AUDIENCE environment variable must be set in the .env file");
                    #[cfg(not(feature = "local-dev"))]
                    let audience = std::env::var("JWT_AUDIENCE")
                        .expect("JWT_AUDIENCE environment variable must be set");
                    audience
                })
                .to_owned(),
        )
        .into(),
        jti: uuid::Uuid::now_v7(),
        scope: scopes,
    };
    let jwt_token = aliri::Jwt::try_from_parts_with_signature(&jwt_headers, &claims, &jwk).unwrap();

    jwt_token
}
