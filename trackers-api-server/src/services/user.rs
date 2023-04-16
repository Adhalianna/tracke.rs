use crate::prelude::*;
use models::{Tracker, TrackerInput};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route(
        "/user/:email/trackers",
        routing::get(get_users_trackers).post(post_to_users_trackers),
    )
}

async fn get_users_trackers(
    State(state): State<AppState>,
    axum::extract::Path(email): axum::extract::Path<EmailAddress>,
) -> Result<Json<Vec<Tracker>>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::trackers::dsl::trackers;
    use db_schema::users::dsl::users;

    let new_user_id: models::types::Uuid = users
        .filter(db_schema::users::columns::email.eq(email))
        .select(db_schema::users::columns::user_id)
        .limit(1)
        .get_result(&mut db_conn)
        .await?;

    let user_trackers: Vec<Tracker> = trackers
        .filter(db_schema::trackers::user_id.eq(new_user_id))
        .get_results(&mut db_conn)
        .await?;

    Ok(Json(user_trackers))
}

async fn post_to_users_trackers(
    State(state): State<AppState>,
    axum::extract::Path(email): axum::extract::Path<EmailAddress>,
    Json(input): Json<TrackerInput>,
) -> Result<([(axum::http::HeaderName, String); 1], Json<Tracker>), ServerError> {
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

    Ok((
        [(
            axum::http::header::LOCATION,
            format!("/api/tracker/{}", &inserted.tracker_id),
        )],
        Json(inserted),
    ))
}
