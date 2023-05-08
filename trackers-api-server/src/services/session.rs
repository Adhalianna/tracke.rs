use crate::{auth::scope::ScopeVariable, prelude::*};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/session",
            routing::post(|| async { axum::response::Redirect::permanent("/api/session/token") }),
        )
        .api_route("/session/token", routing::post(authenticate))
}

/// OAuth2 authentication request. The server supports only resource owner flow
/// at the moment and serves the role of both authorization server and its
/// client.
#[derive(Deserialize, JsonSchema)]
#[serde(tag = "grant_type")]
#[serde(rename_all = "snake_case")]
pub enum AuthReq {
    Password(PasswordGrant),
    Refresh(RefreshRequest),
}

/// OAuth2 authentication request for a resource owner authentication flow.
#[derive(Deserialize, JsonSchema)]
#[serde(rename = "password")]
pub struct PasswordGrant {
    pub username: trackers_models::types::Email,
    pub password: models::types::PasswordInput,
}

/// OAuth2 authentication refresh request.
#[derive(Deserialize, JsonSchema)]
#[serde(rename = "refresh")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// OAuth2 access token which also works as session ID.
#[derive(Serialize, JsonSchema)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: &'static str,
    /// Time until token expiration in seconds,
    pub expires_in: u16,
    pub refresh_token: String,
}

pub async fn authenticate(
    State(state): State<AppState>,
    axum::extract::Form(form): axum::extract::Form<AuthReq>,
) -> Result<Json<AccessToken>, ApiError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::sessions::dsl::sessions;
    use db_schema::users::dsl::users;

    match form {
        AuthReq::Password(form) => {
            let user_search_res: Result<models::db::User, _> = users
                .filter(db_schema::users::email.eq(form.username))
                .first(&mut db_conn)
                .await;
            let user: models::db::User = match user_search_res {
                Ok(user) => user,
                Err(err) => match err {
                    diesel::result::Error::NotFound => {
                        Err(BadRequestError::default().with_msg("email or password not correct"))?
                    }
                    _ => Err(err)?,
                },
            };
            // check password:
            if !form.password.match_with(user.password) {
                Err(BadRequestError::default().with_msg("email or password not correct"))?;
            }
            // generate tokens:
            let access_token = crate::auth::layer::new_token_with_exp_and_scopes(
                30 * 60,
                aliri_oauth2::Scope::default().and(
                    aliri_oauth2::oauth2::ScopeToken::from_string(format!(
                        "{}:{}",
                        crate::auth::scope::UserIdScope::scope_name(),
                        crate::auth::scope::UserIdScope(user.user_id.clone())
                    ))
                    .unwrap(),
                ),
            );
            let refresh_token = crate::auth::layer::new_token_with_exp_and_scopes(
                30 * 60,
                aliri_oauth2::Scope::empty(),
            );
            diesel::insert_into(sessions)
                .values(models::core::Session {
                    user_id: user.user_id,
                    access_token: access_token.clone().take(),
                    refresh_token: refresh_token.clone().take(),
                    started_at: chrono::Utc::now(),
                    valid_until: chrono::Utc::now()
                        .checked_add_signed(chrono::Duration::seconds(30 * 60))
                        .unwrap(),
                })
                .execute(&mut db_conn)
                .await?;
            Ok(Json(AccessToken {
                token_type: "bearer",
                access_token: access_token.take(),
                refresh_token: refresh_token.take(),
                expires_in: 30 * 60,
            }))
        }
        AuthReq::Refresh(refresh) => {
            let session: models::core::Session = sessions
                .filter(db_schema::sessions::refresh_token.eq(&refresh.refresh_token))
                .first(&mut db_conn)
                .await?;

            if session.valid_until.gt(&chrono::Utc::now()) {
                // can still refresh

                // generate tokens:
                let access_token = crate::auth::layer::new_token_with_exp_and_scopes(
                    30 * 60,
                    aliri_oauth2::Scope::empty(),
                );
                let refresh_token = crate::auth::layer::new_token_with_exp_and_scopes(
                    30 * 60,
                    aliri_oauth2::Scope::empty(),
                );

                diesel::update(sessions)
                    .filter(db_schema::sessions::refresh_token.eq(&refresh.refresh_token))
                    .set((
                        db_schema::sessions::refresh_token.eq(refresh_token.clone().take()),
                        db_schema::sessions::access_token.eq(access_token.clone().take()),
                    ))
                    .execute(&mut db_conn)
                    .await?;

                Ok(Json(AccessToken {
                    token_type: "bearer",
                    access_token: access_token.take(),
                    refresh_token: refresh_token.take(),
                    expires_in: 30 * 60,
                }))
            } else {
                // cannot refresh anymore
                Err(BadRequestError::default()
                    .with_msg("the session can no longer be refreshed")
                    .into())
            }
        }
    }
}
