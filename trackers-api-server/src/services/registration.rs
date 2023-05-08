use diesel_async::scoped_futures::ScopedFutureExt;
use models::RegistrationRequest;

use crate::prelude::*;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/registration-request/:email",
            routing::get(get_request),
            |op| op.tag("Registration"),
        )
        .api_route_with(
            "/registration-request/:email/code",
            routing::post(confirm_request),
            |op| op.tag("Registration"),
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
    json: JsonExtract<models::ConfirmationCode>,
) -> Result<ModifiedResource<RegistrationRequest>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let code = json.data;

    // The api error to be returned will be saved outside the transaction.
    // This trick is performed to avoid unnecessary and difficult to understand later trait
    // impl shenanigans.
    //
    // ... still, this is a among first candidates for a refactor
    let mut caught_api_err = None;
    let api_err = &mut caught_api_err;

    let transaction_res = db_conn
        .transaction(|tx| {
            async move {
                use db_schema::registration_requests::dsl::registration_requests;
                use db_schema::users::dsl::users;

                let req: models::db::RegistrationRequest =
                    match registration_requests.find(&email).get_result(tx).await {
                        Ok(req) => req,
                        Err(err) => {
                            return Err(err);
                        }
                    };

                // Request no longer valid
                if req.valid_until < chrono::Utc::now() {
                    // save the api err
                    *api_err = Some(
                        GoneError::default()
                            .with_docs()
                            .with_msg("the request is no longer valid")
                            .into(),
                    );
                    // remove invalid request
                    let clean_up_res = diesel::delete(registration_requests)
                        .filter(db_schema::registration_requests::columns::email.eq(&req.email))
                        .execute(tx)
                        .await;
                    // check and finish early if transaction successful
                    match clean_up_res {
                        Ok(_) => return Ok(None),
                        Err(_) => {
                            return Err(diesel::result::Error::RollbackTransaction);
                        }
                    }
                };

                // Not the code we expected
                if code != req.confirmation_code {
                    // save the api err
                    *api_err = Some(
                        BadRequestError::default()
                            .with_docs()
                            .with_msg("incorrect code")
                            .into(),
                    );
                    return Err(diesel::result::Error::RollbackTransaction);
                };

                let mut updated_req: models::RegistrationRequest = req.clone().into();
                updated_req.confirmed_with_code = true;

                match diesel::insert_into(users)
                    .values(models::db::User {
                        user_id: models::types::Uuid::new(),
                        email: req.email,
                        password: req.password,
                    })
                    .execute(tx)
                    .await
                {
                    Err(err) => {
                        return Err(err);
                    }
                    _ => {}
                };

                match diesel::delete(registration_requests)
                    .filter(db_schema::registration_requests::columns::email.eq(&email))
                    .execute(tx)
                    .await
                {
                    Err(err) => Err(err),
                    _ => Ok(Some(updated_req)),
                }
            }
            .scope_boxed()
        })
        .await;

    // Return early with API error if we had any
    if let Some(caught_api_err) = caught_api_err {
        return Err(caught_api_err);
    };

    // Extract the registration request or handle the transaction errors
    let registration_res = match transaction_res {
        Ok(reg) => reg.unwrap(),
        Err(transaction_err) => Err(transaction_err)?,
    };

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(registration_res),
    })
}
