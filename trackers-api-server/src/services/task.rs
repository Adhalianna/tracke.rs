use crate::{prelude::*, response::ModifiedResource};
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
) -> Result<Resource<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let the_task: db::Task = tasks.find(task_id.clone()).get_result(&mut db_conn).await?;

    Ok(Resource::new(the_task.into())
        .with_links([("checkmark", format!("/api/task/{}/checkmark", task_id))]))
}

async fn make_completed(
    State(state): State<AppState>,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<ModifiedResource<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let updated_task: db::Task = diesel::update(tasks)
        .filter(db_schema::tasks::columns::task_id.eq(task_id.clone()))
        .set(db_schema::tasks::columns::completed_at.eq(diesel::dsl::now))
        .get_result(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: Some(format!("/api/task/{task_id}")),
        resource: Resource::new(updated_task.into()).with_links([
            ("unmark", format!("/api/task/{task_id}/checkmark")),
            ("self", format!("/api/task/{task_id}")),
        ]),
    })
}

async fn make_uncompleted(
    State(state): State<AppState>,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<ModifiedResource<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let updated_task: db::Task = diesel::update(tasks)
        .filter(db_schema::tasks::columns::task_id.eq(task_id.clone()))
        .set(db_schema::tasks::columns::completed_at.eq(Option::<chrono::NaiveDateTime>::None))
        .get_result(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: Some(format!("/api/task/{task_id}")),
        resource: Resource::new(updated_task.into()).with_links([
            ("mark", format!("/api/task/{task_id}/checkmark")),
            ("self", format!("/api/task/{task_id}")),
        ]),
    })
}
