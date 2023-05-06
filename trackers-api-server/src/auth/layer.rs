use super::UserClaims;

const ISSUER: &str = "authority";
const AUDIENCE: &str = "my_api";
const KEY_ID: &str = "test key";
const SHARED_SECRET: &[u8] = b"test";

pub async fn require_jwt<B: axum::body::HttpBody>(
    request: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        axum::http::header::WWW_AUTHENTICATE,
        axum::http::HeaderValue::from_str("Bearer realm=\"tracke.rs\" ")
            .expect("failed to save JWT into response headers"),
    );

    Ok(response)
}

pub fn validator() -> &'static aliri::jwt::CoreValidator {
    static VALID: once_cell::sync::OnceCell<aliri::jwt::CoreValidator> =
        once_cell::sync::OnceCell::new();

    VALID.get_or_init(|| {
        aliri::jwt::CoreValidator::default()
            .add_approved_algorithm(aliri::jwa::Algorithm::HS256)
            .add_allowed_audience(aliri::jwt::Audience::from_static(AUDIENCE))
            .require_issuer(aliri::jwt::Issuer::from_static(ISSUER))
    })
}

pub fn authority() -> &'static aliri_oauth2::Authority {
    static AUTH: once_cell::sync::OnceCell<aliri_oauth2::Authority> =
        once_cell::sync::OnceCell::new();
    AUTH.get_or_init(|| {
        let key = aliri::Jwk::from(aliri::jwa::Hmac::new(SHARED_SECRET))
            .with_algorithm(aliri::jwa::Algorithm::HS256)
            .with_key_id(aliri::jwk::KeyId::from_static(KEY_ID));

        let mut jwks = aliri::Jwks::default();
        jwks.add_key(key);

        aliri_oauth2::Authority::new(jwks, validator().clone())
    })
}

pub fn authorizer<B: axum::body::HttpBody + Default>(
) -> aliri_tower::Oauth2Authorizer<crate::auth::UserClaims, aliri_tower::TerseErrorHandler<B>> {
    aliri_tower::Oauth2Authorizer::new()
        .with_claims::<crate::auth::UserClaims>()
        .with_terse_error_handler::<B>()
}

pub fn new_token_with_exp_and_scopes(
    lifetime_in_seconds: u64,
    scopes: aliri_oauth2::Scope,
) -> aliri::Jwt {
    let jwt_headers = aliri::jwt::BasicHeaders::with_key_id(
        aliri::jwa::Algorithm::HS256,
        aliri::jwk::KeyId::from_static(KEY_ID),
    );
    let jwk = aliri::Jwk::from(aliri::jwa::Hmac::new(SHARED_SECRET))
        .with_algorithm(aliri::jwa::Algorithm::HS256)
        .with_key_id(aliri::jwk::KeyId::from_static(KEY_ID));
    let claims = UserClaims {
        exp: aliri_clock::UnixTime::from({
            std::time::SystemTime::now()
                .checked_add(std::time::Duration::from_secs(lifetime_in_seconds))
                .unwrap()
        }),
        iss: aliri::jwt::Issuer::new(ISSUER.to_owned()),
        aud: aliri::jwt::Audience::new(AUDIENCE.to_owned()).into(),
        jti: uuid::Uuid::now_v7(),
        scope: scopes,
    };
    let jwt_token = aliri::Jwt::try_from_parts_with_signature(&jwt_headers, &claims, &jwk).unwrap();

    jwt_token
}
