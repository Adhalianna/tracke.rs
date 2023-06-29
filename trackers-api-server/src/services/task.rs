use crate::{prelude::*, response::ModifiedResource};
use models::{
    db::{self},
    Task,
};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/task/:task_id",
            routing::get_with(get_one_task, |op| op.summary("Fetch the task"))
                .patch_with(patch_task, |op| op.summary("Update the task")),
            |op| op.tag("Task Management"),
        )
        .api_route_with(
            "/task/:task_id/checkmark",
            routing::post_with(make_completed, |op| op.summary("Mark the task done"))
                .put_with(make_completed, |op| op.summary("Mark the task done"))
                .delete_with(make_uncompleted, |op| op.summary("Unmark the task as done")),
            |op| op.tag("Task Management"),
        )
        .layer(crate::auth::layer::authorizer().jwt_layer(crate::auth::layer::authority().clone()))
}

// Attach appropiate links to the task resource
fn task_links(task: &models::db::Task) -> Vec<(&'static str, String)> {
    let mut links = Vec::new();
    if task.completed_at.is_some() {
        links.push(("unmark", format!("/api/task/{}/checkmark", task.task_id)));
    } else {
        links.push(("checkmark", format!("/api/task/{}/checkmark", task.task_id)))
    }
    if task.list.is_some() {
        links.extend([
            ("replace list", format!("/api/task/{}/list", task.task_id)),
            ("delete list", format!("/api/task/{}/list", task.task_id)),
        ])
    } else {
        links.push(("attach list", format!("/api/task/{}/list", task.task_id)))
    }
    links.push(("tracker", format!("/api/tracker/{}", task.tracker_id)));
    links.push(("self", format!("/api/task/{}", task.task_id)));

    links
}

async fn get_one_task(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<Resource<Task>, ApiError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::dsl::tasks;

    let (the_task, task_user_id): (models::db::Task, models::types::Uuid) = tasks
        .inner_join(db_schema::trackers::table)
        .filter(db_schema::tasks::task_id.eq(task_id.clone()))
        // .filter(db_schema::trackers::user_id.eq(user_id.0))
        .select((db_schema::tasks::all_columns, db_schema::trackers::user_id))
        .first::<(db::Task, models::types::Uuid)>(&mut db_conn)
        .await?;

    if task_user_id != user_id.0 {
        Err(ForbiddenError::default().with_msg("no access to the selected task"))?;
    }

    let links = task_links(&the_task);
    let resource: Resource<Task> = Resource::new(the_task.into());

    Ok(resource.with_links(links))
}

async fn patch_task(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
    json: JsonExtract<models::core::TaskPatch>,
) -> Result<ModifiedResource<Task>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let patch = json.extract();

    if let Some(json_task_id) = &patch.task_id {
        if *json_task_id != task_id {
            Err(ConflictError::default().with_msg("mismatch between the task id provided in the body and in the path of the request").with_docs())?;
        }
    }

    if let Some(tracker_id) = &patch.tracker_id {
        let res = db_schema::trackers::table
            .filter(
                db_schema::trackers::user_id
                    .eq(user_id.0)
                    .and(db_schema::trackers::tracker_id.eq(tracker_id)),
            )
            .execute(&mut db_conn)
            .await?;
        if res < 1 {
            Err(ForbiddenError::default()
                .with_msg("no access to the selected tracker")
                .with_docs())?;
        }
    }

    let updated: models::db::Task = diesel::update(db_schema::tasks::table)
        .filter(db_schema::tasks::task_id.eq(task_id))
        .set(models::db::TaskPatch::from(dbg!(patch)))
        .get_result(&mut db_conn)
        .await?;

    let links = task_links(&updated);

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(updated.into()).with_links(links),
    })
}

async fn make_completed(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<ModifiedResource<Task>, ApiError> {
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
        Err(ForbiddenError::default().with_msg("no access to selected task"))?;
    }

    let updated_task: db::Task = diesel::update(tasks)
        .filter(db_schema::tasks::columns::task_id.eq(task_id.clone()))
        .set(db_schema::tasks::columns::completed_at.eq(chrono::Utc::now()))
        .get_result(&mut db_conn)
        .await?;

    let links = task_links(&updated_task);

    Ok(ModifiedResource {
        location: Some(format!("/api/task/{task_id}")),
        resource: Resource::new(updated_task.into()).with_links(links),
    })
}

async fn make_uncompleted(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<ModifiedResource<Task>, ApiError> {
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
        Err(ForbiddenError::default().with_msg("no access to selected task"))?;
    }

    let updated_task: db::Task = diesel::update(tasks)
        .filter(db_schema::tasks::columns::task_id.eq(task_id.clone()))
        .set(db_schema::tasks::columns::completed_at.eq(Option::<chrono::NaiveDateTime>::None))
        .get_result(&mut db_conn)
        .await?;

    let links = task_links(&updated_task);

    Ok(ModifiedResource {
        location: Some(format!("/api/task/{task_id}")),
        resource: Resource::new(updated_task.into()).with_links(links),
    })
}
