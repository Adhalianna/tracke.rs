use crate::db::models::Task;
use crate::prelude::*;

pub fn hello() -> ApiRouter<AppState> {
    ApiRouter::new().api_route("/hello", routing::get(get_hello).post(post_hello_task))
}

pub async fn get_hello() -> Result<&'static str, ServerError> {
    Ok("Hello World!")
}

pub async fn post_hello_task(State(state): State<AppState>) -> Result<Json<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;

    use crate::db::schema::tasks::dsl::*;

    let new_task_id = uuid::Uuid::now_v7();

    diesel::insert_into(tasks)
        .values(Task {
            task_id: new_task_id.clone(),
            user_id: None,
            group_id: None,
            title: "Hello World!".to_owned(),
            description: None,
            time_estimate: None,
            soft_deadline: None,
            hard_deadline: None,
            tags: None,
        })
        .execute(&mut db_conn)
        .await?;

    let inserted: Task = tasks.find(new_task_id).first(&mut db_conn).await?;

    Ok(Json(inserted))
}
