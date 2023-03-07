use crate::db::models::Hello;
use crate::prelude::*;

pub fn hello() -> Router<AppState> {
    Router::new().route("/hello", routing::get(get_hello))
}

#[utoipa::path(get, path = "/api/hello", responses(
    (status = 200, description = "Server successfully fetched first key-value pair from db table 'hello'", body = Hello),
    (status = 500, description = "Server failed to fetch any data from db")
))]
pub async fn get_hello(State(state): State<AppState>) -> Result<axum::Json<Hello>, ServerError> {
    use crate::db::schema::hello::dsl::*;

    let mut db_conn = state.db.get().await?;

    let res = hello.first::<Hello>(&mut db_conn).await?;

    Ok(Json::from(res))
}
