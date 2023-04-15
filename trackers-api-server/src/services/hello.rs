use crate::prelude::*;
use models::{db, Task, TaskInput};

pub fn hello() -> ApiRouter<AppState> {
    ApiRouter::new().api_route("/hello", routing::get(get_hello).post(post_hello_task))
}

pub async fn get_hello(State(state): State<AppState>) -> Result<Json<Vec<Task>>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::*;

    let all_tasks: Vec<db::Task> = tasks.load(&mut db_conn).await?;

    Ok(Json(
        all_tasks.into_iter().map(|t: db::Task| t.into()).collect(),
    ))
}

pub async fn post_hello_task(
    State(state): State<AppState>,
    Json(task): Json<TaskInput>,
) -> Result<Json<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;

    use db_schema::tasks::dsl::*;

    let new_task_id = task.task_id.unwrap_or(uuid::Uuid::now_v7());

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
