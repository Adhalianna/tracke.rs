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
        .layer(crate::auth::layer::authorizer().jwt_layer(crate::auth::layer::authority().clone()))
}

// TODO: protect from unauthorized use (check if user, from session, owns the task)
async fn get_one_task(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<Resource<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let the_task: db::Task = tasks
        .inner_join(db_schema::trackers::table)
        .filter(db_schema::tasks::task_id.eq(task_id.clone()))
        .filter(db_schema::trackers::user_id.eq(user_id.0))
        .select(db_schema::tasks::all_columns)
        .first::<db::Task>(&mut db_conn)
        .await?;

    let resource: Resource<Task> = Resource::new(the_task.into());

    if resource.data.checkmarked {
        Ok(resource.with_links([("unmark", format!("/api/task/{}/checkmark", task_id))]))
    } else {
        Ok(resource.with_links([("checkmark", format!("/api/task/{}/checkmark", task_id))]))
    }
}

async fn make_completed(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<ModifiedResource<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    // Check if owned and exists:
    let res = tasks
        .inner_join(db_schema::trackers::table)
        .filter(db_schema::tasks::task_id.eq(task_id.clone()))
        .filter(db_schema::trackers::user_id.eq(user_id.0))
        .execute(&mut db_conn)
        .await?;
    if res < 1 {
        todo!()
    }

    let updated_task: db::Task = diesel::update(tasks)
        .filter(db_schema::tasks::columns::task_id.eq(task_id.clone()))
        .set(db_schema::tasks::columns::completed_at.eq(chrono::Utc::now()))
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
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<ModifiedResource<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    // Check if owned and exists:
    let res = tasks
        .inner_join(db_schema::trackers::table)
        .filter(db_schema::tasks::task_id.eq(task_id.clone()))
        .filter(db_schema::trackers::user_id.eq(user_id.0))
        .execute(&mut db_conn)
        .await?;
    if res < 1 {
        todo!()
    }

    let updated_task: db::Task = diesel::update(tasks)
        .filter(db_schema::tasks::columns::task_id.eq(task_id.clone()))
        .set(db_schema::tasks::columns::completed_at.eq(Option::<chrono::NaiveDateTime>::None))
        .get_result(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: Some(format!("/api/task/{task_id}")),
        resource: Resource::new(updated_task.into()).with_links([
            ("checkmark", format!("/api/task/{task_id}/checkmark")),
            ("self", format!("/api/task/{task_id}")),
        ]),
    })
}
