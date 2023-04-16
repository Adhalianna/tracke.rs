use crate::prelude::*;
use models::{
    db::{self},
    Task, TaskInput,
};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route("/task/:task_id", routing::get(get_one))
}

pub async fn get_one(
    State(state): State<AppState>,
    axum::extract::Path(task_id): axum::extract::Path<models::types::Uuid>,
) -> Result<Json<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let the_task: db::Task = tasks.find(task_id).get_result(&mut db_conn).await?;

    Ok(Json(the_task.into()))
}

pub async fn post(
    State(state): State<AppState>,
    Json(task): Json<TaskInput>,
) -> Result<Json<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;

    use db_schema::tasks::dsl::*;

    let new_task_id = task.task_id.unwrap_or(uuid::Uuid::now_v7().into());

    diesel::insert_into(tasks)
        .values(db::Task {
            task_id: new_task_id.clone(),
            tracker_id: task.tracker_id,
            title: task.title,
            description: task.description,
            completed_at: {
                if task.completed {
                    Some(chrono::Local::now().naive_local())
                } else {
                    None
                }
            },
            time_estimate: task.time_estimate,
            soft_deadline: task.soft_deadline,
            hard_deadline: task.hard_deadline,
            tags: task.tags,
        })
        .execute(&mut db_conn)
        .await?;

    let inserted: db::Task = tasks.find(new_task_id).first(&mut db_conn).await?;

    Ok(Json(inserted.into()))
}
