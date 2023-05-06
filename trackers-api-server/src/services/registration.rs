use diesel_async::scoped_futures::ScopedFutureExt;
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

                if req.valid_until < chrono::Utc::now() {
                    //TODO: return client error

                    // Err(anyhow::anyhow!("too old"))?;
                    return Err(diesel::result::Error::RollbackTransaction);
                };
                if code != req.confirmation_code {
                    //TODO: return client error

                    // Err(anyhow::anyhow!("bad code"))?;
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
                    Ok(_) => {}
                    Err(err) => {
                        return Err(err);
                    }
                };

                match diesel::delete(registration_requests)
                    .filter(db_schema::registration_requests::columns::email.eq(&email))
                    .execute(tx)
                    .await
                {
                    Ok(_) => Ok(updated_req),
                    Err(err) => Err(err),
                }
            }
            .scope_boxed()
        })
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(transaction_res),
    })
}
