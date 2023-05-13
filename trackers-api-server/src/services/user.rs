use crate::prelude::*;
use models::{RegistrationRequest, Tracker, TrackerInput, UserCreation};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/user/:email/trackers",
            routing::get(get_users_trackers)
                .post(post_to_users_trackers)
                .layer(
                    crate::auth::layer::authorizer()
                        .jwt_layer(crate::auth::layer::authority().clone()),
                ),
            |op| op.tag("Task Management"),
        )
        .api_route_with(
            "/user/:email/tasks",
            routing::get(get_all_user_tasks).layer(
                crate::auth::layer::authorizer().jwt_layer(crate::auth::layer::authority().clone()),
            ),
            |op| op.tag("Task Management"),
        )
        .api_route_with("/users", routing::post(start_user_registaration), |op| {
            op.tag("Registration")
        })
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

/// Modifies the default json rejection to be a bit more user friendly
/// It is a little bit ugly patch tho
pub struct UserCreationRejection;
impl crate::json::FromJsonRejection for UserCreationRejection {
    type NewRejection = BadRequestError;

    fn from_rejection(rejection: axum::extract::rejection::JsonRejection) -> Self::NewRejection {
        match rejection {
            axum::extract::rejection::JsonRejection::JsonDataError(err) => {
                let mut msg = err
                    .body_text()
                    .strip_prefix(&(err.to_string() + ": "))
                    .unwrap()
                    .to_owned();
                if let Some(passwd_msg) = msg.strip_prefix("password: ") {
                    msg = String::new() + "password too weak; " + passwd_msg;
                    msg = msg.split_at(msg.find(" at line").unwrap()).0.to_owned();
                };
                BadRequestError::default().with_docs().with_msg(msg)
            }
            _ => crate::json::ApiJsonRejection::from_rejection(rejection),
        }
    }
}

impl aide::OperationOutput for UserCreationRejection {
    type Inner = BadRequestError;
    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        operation: &mut openapi::Operation,
    ) -> Vec<(Option<u16>, openapi::Response)> {
        crate::json::ApiJsonRejection::inferred_responses(ctx, operation)
        // TODO: provide stricter examples instead
    }
}

async fn start_user_registaration(
    State(state): State<AppState>,
    json: JsonExtract<UserCreation, UserCreationRejection>,
) -> Result<CreatedResource<RegistrationRequest>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let new_user = json.extract();

    // Check if ToS accepted
    if !new_user.accepted_tos {
        Err(BadRequestError::default()
            .with_msg("terms of service must be accepted for a user to create an account"))?;
    }

    // Check if email is free and can be used to create a new account
    let existing_users_with_email = db_schema::users::table
        .filter(db_schema::users::email.eq(&new_user.email))
        .execute(&mut db_conn)
        .await?;
    if existing_users_with_email > 0 {
        Err(ConflictError::default().with_msg("email already taken by another account"))?;
    }

    // Generate confirmation code
    let code = models::ConfirmationCode::new();
    #[cfg(debug_assertions)]
    println!("registration code {code} has been generated");

    // Create registration request
    let req: models::db::RegistrationRequest =
        diesel::insert_into(db_schema::registration_requests::table)
            .values(models::db::RegistrationRequest {
                issued_at: chrono::offset::Utc::now(),
                valid_until: chrono::offset::Utc::now()
                    .checked_add_signed(chrono::Duration::minutes(10))
                    .unwrap(),
                email: new_user.email.clone(),
                password: new_user.password.into_storeable(),
                confirmation_code: code.clone().into(),
            })
            .get_result(&mut db_conn)
            .await?;

    // Send the code through an email with SendGrid
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
) -> Result<Resource<Vec<Tracker>>, ApiError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::trackers::dsl::trackers;

    let user_trackers: Vec<Tracker> = trackers
        .inner_join(db_schema::users::table)
        .select(db_schema::trackers::all_columns)
        .filter(
            db_schema::trackers::user_id
                .eq(user_id.0)
                .and(db_schema::users::email.eq(email)),
        )
        .get_results(&mut db_conn)
        .await?;

    if user_trackers.is_empty() {
        Err(NotFoundError::default().with_msg("failed to find any accessible trackers"))?;
    }

    Ok(Resource::new(user_trackers))
}

async fn post_to_users_trackers(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(email): axum::extract::Path<EmailAddress>,
    json: JsonExtract<TrackerInput>,
) -> Result<CreatedResource<Tracker>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let input = json.data;

    let new_tracker_id = input.tracker_id.unwrap_or(uuid::Uuid::now_v7().into());

    if let Some(input_user_id) = input.user_id {
        if input_user_id != user_id.0 {
            Err(ForbiddenError::default()
                .with_msg("cannot add trackers for such user from current session"))?;
        };
    };

    let user_exists = db_schema::users::table
        .filter(
            db_schema::users::email
                .eq(email)
                .and(db_schema::users::user_id.eq(&user_id.0)),
        )
        .execute(&mut db_conn)
        .await?;
    if user_exists < 1 {
        Err(ForbiddenError::default()
            .with_msg("cannot add trackers for such user from current session"))?;
    }

    let inserted: Tracker = diesel::insert_into(db_schema::trackers::dsl::trackers)
        .values(Tracker {
            tracker_id: new_tracker_id,
            user_id: user_id.0,
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

async fn get_all_user_tasks(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    query: Option<QsQuery<crate::query_param::TasksQuery>>,
    axum::extract::Path(email): axum::extract::Path<EmailAddress>,
) -> Result<Resource<Vec<models::core::task::Task>>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let mut tasks_query = db_schema::trackers::table
        .filter(db_schema::trackers::columns::user_id.eq(user_id.0))
        .inner_join(db_schema::tasks::table)
        .select(db_schema::tasks::all_columns)
        .into_boxed();

    if let Some(query) = query {
        dbg!(&query);
        tasks_query = tasks_query.filter(query.into_join_filters());
    }

    let trackers_tasks: Vec<models::db::Task> = tasks_query.load(&mut db_conn).await?;

    Ok(Resource::new({
        trackers_tasks.into_iter().map(|t| t.into()).collect()
    }))
}
