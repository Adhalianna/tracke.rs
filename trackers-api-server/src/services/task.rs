use crate::prelude::*;
use models::{
    db::{self},
    Task,
};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/task/:task_id", routing::get(get_one_task))
        .api_route(
            "/task/:task_id/checkmark",
            routing::post(make_completed)
                .put(make_completed)
                .delete(make_uncompleted),
        )
}

// TODO: protect from unauthorized use (check if user, from session, owns the task)
async fn get_one_task(
    State(state): State<AppState>,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<Json<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let the_task: db::Task = tasks.find(task_id).get_result(&mut db_conn).await?;

    Ok(Json(the_task.into()))
}

async fn make_completed(
    State(state): State<AppState>,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<([(axum::http::HeaderName, String); 1], Json<Task>), ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let updated_task: db::Task = diesel::update(tasks)
        .filter(db_schema::tasks::columns::task_id.eq(task_id))
        .set(db_schema::tasks::columns::completed_at.eq(diesel::dsl::now))
        .get_result(&mut db_conn)
        .await?;

    Ok((
        [(
            axum::http::header::LOCATION,
            format!("/api/task/{}", &updated_task.task_id),
        )],
        Json(updated_task.into()),
    ))
}

async fn make_uncompleted(
    State(state): State<AppState>,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<([(axum::http::HeaderName, String); 1], Json<Task>), ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let updated_task: db::Task = diesel::update(tasks)
        .filter(db_schema::tasks::columns::task_id.eq(task_id))
        .set(db_schema::tasks::columns::completed_at.eq(Option::<chrono::NaiveDateTime>::None))
        .get_result(&mut db_conn)
        .await?;

    Ok((
        [(
            axum::http::header::LOCATION,
            format!("/api/task/{}", &updated_task.task_id),
        )],
        Json(updated_task.into()),
    ))
}
