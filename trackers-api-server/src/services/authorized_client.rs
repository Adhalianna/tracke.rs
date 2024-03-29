use models::{AuthorisedClient, AuthorisedClientFull};

use crate::prelude::*;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/user/:email/authorised_clients",
            routing::get_with(get_authorised_clients, |op| {
                op.summary("Fetch all authorised clients")
            })
            .post_with(authorize_new_client, |op| {
                op.summary("Authorise a new client")
            }),
            |op| op.tag("Authorizing Client Applications"),
        )
        .route(
            "/user/:email/authorised_client/:client_id",
            routing::get(use_post_instead),
        )
        .api_route_with(
            "/user/:email/authorised_client/:client_id",
            routing::post_with(get_full_athorised_client, |op| {
                op.summary("Fetch authorised client details")
                    .description("Note that issueing this request with GET method will result in an error. This is because we do not want to have that request to be stored in cache of any browser software.")
            })
            .delete_with(remove_authorisation, |op| op.summary("Unauthorise application")),
            |op| op.tag("Authorizing Client Applications"),
        )
        .layer(crate::auth::layer::authorizer().jwt_layer(crate::auth::layer::authority().clone()))
}

async fn authorize_new_client(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(email): axum::extract::Path<EmailAddress>, // whacky thing here, that email is not checked for validity with the user_id
    json: JsonExtract<models::ClientCredentialsRequest>,
) -> Result<CreatedResource<models::AuthorisedClientFull>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let client_request = json.extract();

    if client_request.user_id != user_id.0 {
        return Err(ForbiddenError::default()
            .with_msg("session does not match the user_id provided in the payload")
            .with_docs()
            .into());
    }

    let authorised_client = client_request.to_authorised_client();
    diesel::insert_into(db_schema::authorised_clients::table)
        .values(&authorised_client)
        .execute(&mut db_conn)
        .await?;

    Ok(CreatedResource {
        location: format!(
            "/user/{email}/authorised_client/{client_id}",
            client_id = &authorised_client.client_id
        ),
        resource: Resource::new(authorised_client),
    })
}

async fn use_post_instead(
    axum::extract::Path((email, client_id)): axum::extract::Path<(EmailAddress, String)>,
) -> ApiError {
    BadRequestError::default()
        .with_msg(
            "POST request should be used to avoid accidentially saving sensitive details in cache",
        )
        .with_docs()
        .with_links([(
            "authorised client details",
            format!("/api/user/{email}/authorised_client/{client_id}"),
        )])
        .into()
}

async fn get_full_athorised_client(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path((email, client_id)): axum::extract::Path<(EmailAddress, String)>,
) -> Result<Resource<AuthorisedClientFull>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let authorised_client = db_schema::authorised_clients::table
        .inner_join(db_schema::users::table)
        .filter(
            db_schema::users::user_id
                .eq(user_id.0)
                .and(db_schema::users::email.eq(email))
                .and(db_schema::authorised_clients::client_id.eq(client_id)),
        )
        .select(db_schema::authorised_clients::all_columns)
        .get_result(&mut db_conn)
        .await?;

    Ok(Resource::new(authorised_client))
}

async fn remove_authorisation(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path((email, client_id)): axum::extract::Path<(EmailAddress, String)>,
) -> Result<DeletedResource, ApiError> {
    let mut db_conn = state.db.get().await?;

    diesel::delete(db_schema::authorised_clients::table)
        .filter(
            db_schema::users::table
                .filter(db_schema::users::email.eq(email))
                .select(db_schema::users::user_id)
                .single_value()
                .eq(user_id.0),
        )
        .filter(db_schema::authorised_clients::client_id.eq(client_id))
        .execute(&mut db_conn)
        .await?;

    Ok(DeletedResource::default())
}

async fn get_authorised_clients(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(email): axum::extract::Path<EmailAddress>,
) -> Result<Resource<Vec<models::AuthorisedClient>>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let authorised_clients: Vec<models::AuthorisedClientFull> =
        db_schema::authorised_clients::table
            .inner_join(db_schema::users::table)
            .filter(
                db_schema::users::email
                    .eq(email)
                    .and(db_schema::users::user_id.eq(user_id.0)),
            )
            .select(db_schema::authorised_clients::all_columns)
            .get_results(&mut db_conn)
            .await?;

    Ok(Resource::new(
        authorised_clients
            .into_iter()
            .map(|full| full.into())
            .collect(),
    ))
}
