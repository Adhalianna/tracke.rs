use crate::prelude::*;
use models::{RegistrationRequest, Tracker, TrackerInput, UserCreation};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/user/:email/trackers",
            routing::get(get_users_trackers)
                .post(post_to_users_trackers)
                .layer(
                    crate::auth::layer::authorizer()
                        .jwt_layer(crate::auth::layer::authority().clone()),
                ),
        )
        .api_route("/users", routing::post(start_user_registaration))
}

async fn send_registration_code_mail(
    generated_code: &models::core::ConfirmationCode,
    receiver: &models::types::Email,
) -> Result<reqwest::Response, reqwest::Error> {
    let sendgrid_api_key = {
        #[cfg(feature = "local-dev")]
        let key = dotenvy::var("SENDGRID_API_KEY")
            .expect("SENDGRID_API_KEY environment variable must be set");
        #[cfg(not(feature = "local-dev"))]
        let key = std::env::var("SENDGRID_API_KEY")
            .expect("SENDGRID_API_KEY environment variable must be set");
        key
    };
    let sendgrid_template_id = {
        #[cfg(feature = "local-dev")]
        let id = dotenvy::var("SENDGRID_REG_CODE_TEMPLATE_ID")
            .expect("SENDGRID_REG_CODE_TEMPLATE_ID environment variable must be set");
        #[cfg(not(feature = "local-dev"))]
        let id = std::env::var("SENDGRID_REG_CODE_TEMPLATE_ID")
            .expect("SENDGRID_REG_CODE_TEMPLATE_ID environment variable must be set");
        id
    };
    let sendgrid_sender = {
        #[cfg(feature = "local-dev")]
        let address = dotenvy::var("SENDGRID_MAIL_SENDER")
            .expect("SENDGRID_MAIL_SENDER environment variable must be set");
        #[cfg(not(feature = "local-dev"))]
        let address = std::env::var("SENDGRID_MAIL_SENDER")
            .expect("SENDGRID_MAIL_SENDER environment variable must be set");
        address
    };

    let body = format!(
        r#"{{"from":{{"email":"{sendgrid_sender}"}},"personalizations":[{{"to":[{{"email":"{receiver}"}}],"dynamic_template_data":{{"code":"{generated_code}"}}}}],"template_id":"{sendgrid_template_id}"}}"#
    );

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(sendgrid_api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    res
}

async fn start_user_registaration(
    State(state): State<AppState>,
    Json(new_user): Json<models::UserCreation>,
) -> Result<CreatedResource<RegistrationRequest>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::registration_requests::dsl::registration_requests;

    let code = models::ConfirmationCode::new();

    #[cfg(debug_assertions)]
    println!("registration code {code} has been generated");

    let req: models::db::RegistrationRequest = diesel::insert_into(registration_requests)
        .values(models::db::RegistrationRequest {
            issued_at: chrono::offset::Utc::now(),
            valid_until: chrono::offset::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(10))
                .ok_or(anyhow::anyhow!("failed to add 10 minutes to the timestamp to construct the deadline for confirmation"))?,
            email: new_user.email.clone(),
            password: new_user.password.into_storeable(),
            confirmation_code: code.clone().into(),
        })
        .get_result(&mut db_conn)
        .await?;

    send_registration_code_mail(&code, &new_user.email).await?;

    Ok(CreatedResource {
        location: format!("/api/registration-request/{}", &new_user.email),
        resource: Resource::new(req.into()).with_links([
            (
                "self",
                format!("/api/registration-request/{}", &new_user.email),
            ),
            (
                "confirm",
                format!("/api/registration-request/{}/code", &new_user.email),
            ),
        ]),
    })
}

async fn get_users_trackers(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(email): axum::extract::Path<EmailAddress>,
) -> Result<Resource<Vec<Tracker>>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::trackers::dsl::trackers;

    // TODO: Check email

    let user_trackers: Vec<Tracker> = trackers
        .filter(db_schema::trackers::user_id.eq(user_id.0))
        .get_results(&mut db_conn)
        .await?;

    Ok(Resource::new(user_trackers))
}

async fn post_to_users_trackers(
    State(state): State<AppState>,
    axum::extract::Path(email): axum::extract::Path<EmailAddress>,
    Json(input): Json<TrackerInput>,
) -> Result<CreatedResource<Tracker>, ServerError> {
    let mut db_conn = state.db.get().await?;
    let new_tracker_id = input.tracker_id.unwrap_or(uuid::Uuid::now_v7().into());
    let user_uuid = {
        if let Some(id) = input.user_id {
            id
        } else {
            let id = db_schema::users::dsl::users
                .filter(db_schema::users::columns::email.eq(email))
                .limit(1)
                .select(db_schema::users::columns::user_id)
                .get_result(&mut db_conn)
                .await?;
            id
        }
    };

    let inserted: Tracker = diesel::insert_into(db_schema::trackers::dsl::trackers)
        .values(Tracker {
            tracker_id: new_tracker_id,
            user_id: user_uuid,
            name: input.name,
            is_default: false,
        })
        .get_result(&mut db_conn)
        .await?;

    Ok(CreatedResource {
        location: format!("/api/tracker/{}", &inserted.tracker_id),
        resource: Resource::new(inserted),
    })
}
