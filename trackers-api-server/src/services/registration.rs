use models::RegistrationRequest;

use crate::prelude::*;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/registration-request/:email", routing::get(get_request))
        .api_route(
            "/registration-request/:email/code",
            routing::post(confirm_request),
        )
}

async fn get_request(
    State(state): State<AppState>,
    Path(email): Path<models::types::Email>,
) -> Result<Resource<RegistrationRequest>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::registration_requests::dsl::registration_requests;

    let req: models::db::RegistrationRequest = registration_requests
        .find(&email)
        .get_result(&mut db_conn)
        .await?;

    Ok(Resource::new(req.into()).with_links([
        ("self", format!("/api/registration-request/{}", &email)),
        (
            "confirm",
            format!("/api/registration-request/{}/code", &email),
        ),
    ]))
}

async fn confirm_request(
    State(state): State<AppState>,
    Path(email): Path<models::types::Email>,
    Json(code): Json<models::ConfirmationCode>,
) -> Result<ModifiedResource<RegistrationRequest>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::registration_requests::dsl::registration_requests;

    let req: models::db::RegistrationRequest = registration_requests
        .find(&email)
        .get_result(&mut db_conn)
        .await?;

    if req.valid_until < chrono::Utc::now() {
        //TODO: return client error
        diesel::delete(registration_requests)
            .filter(db_schema::registration_requests::columns::valid_until.lt(chrono::Utc::now()))
            .execute(&mut db_conn)
            .await?;
        Err(anyhow::anyhow!("too old"))?;
    };
    if code != req.confirmation_code {
        //TODO: return client error
        Err(anyhow::anyhow!("bad code"))?;
    };
    let mut updated_req: models::RegistrationRequest = req.into();
    updated_req.confirmed_with_code = true;

    diesel::delete(registration_requests)
        .filter(db_schema::registration_requests::columns::email.eq(&email))
        .execute(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(updated_req),
    })
}
