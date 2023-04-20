use crate::prelude::*;
use models::{db, Task, TaskInput, Tracker};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/tracker/:tracker_id",
            routing::get(get_one_tracker)
                .put(replace_tracker)
                .delete(delete_tracker),
        )
        .api_route(
            "/tracker/:tracker_id/tasks",
            routing::get(get_trackers_tasks).post(post_to_tracker_a_task),
        )
}

async fn get_one_tracker(
    State(state): State<AppState>,
    axum::extract::Path(tracker_id): axum::extract::Path<Base62Uuid>,
) -> Result<Resource<Tracker>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::trackers::dsl::trackers;

    let the_tracker: Tracker = trackers
        .find(tracker_id.clone())
        .get_result(&mut db_conn)
        .await?;

    Ok(Resource::new(the_tracker)
        .with_links([("tasks", format!("/api/tracker/{tracker_id}/tasks"))]))
}

async fn replace_tracker(
    State(state): State<AppState>,
    axum::extract::Path(tracker_id): axum::extract::Path<Base62Uuid>,
    Json(input): Json<Tracker>,
) -> Result<ModifiedResource<Tracker>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::trackers::dsl::trackers;

    if tracker_id != input.tracker_id {
        // TODO: Error on incosistent state
    }

    let tracker: Tracker = diesel::insert_into(trackers)
        .values(input)
        .get_result(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(tracker)
            .with_links([("tasks", format!("/api/tracker/{tracker_id}/tasks"))]),
    })
}

async fn delete_tracker(
    State(state): State<AppState>,
    axum::extract::Path(tracker_id): axum::extract::Path<Base62Uuid>,
) -> Result<DeletedResource, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::trackers::{columns, dsl::trackers};

    let affected = diesel::delete(trackers)
        .filter(columns::tracker_id.eq(tracker_id))
        .execute(&mut db_conn)
        .await?;

    if affected < 1 {
        // NOTE: We assume it is impossible to get more than 1 row affected as tracker_id
        // is a primary key and filtering by it should cause at most one row affected.

        // TODO: Error, not found
    }

    Ok(DeletedResource {
        ..Default::default()
    })
}

async fn get_trackers_tasks(
    State(state): State<AppState>,
    axum::extract::Path(the_tracker_id): axum::extract::Path<Base62Uuid>,
) -> Result<Resource<Vec<Task>>, ServerError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::tasks::columns::tracker_id;
    use db_schema::tasks::dsl::tasks;

    let trackers_tasks: Vec<db::Task> = tasks
        .filter(tracker_id.eq(the_tracker_id))
        .load(&mut db_conn)
        .await?;

    Ok(Resource::new({
        trackers_tasks.into_iter().map(|t| t.into()).collect()
    }))
}

async fn post_to_tracker_a_task(
    State(state): State<AppState>,
    axum::extract::Path(the_tracker_id): axum::extract::Path<Base62Uuid>,
    Json(input): Json<TaskInput>,
) -> Result<CreatedResource<Task>, ServerError> {
    let mut db_conn = state.db.get().await?;

    use db_schema::tasks::dsl::*;

    let new_task_id = input.task_id.unwrap_or(uuid::Uuid::now_v7().into());

    if the_tracker_id != input.tracker_id {
        // TODO: Error on incosistent state
    }

    diesel::insert_into(tasks)
        .values(db::Task {
            task_id: new_task_id.clone(),
            tracker_id: input.tracker_id,
            title: input.title,
            description: input.description,
            completed_at: {
                if input.checkmarked {
                    Some(chrono::Local::now().naive_local())
                } else {
                    None
                }
            },
            time_estimate: input.time_estimate,
            soft_deadline: input.soft_deadline,
            hard_deadline: input.hard_deadline,
            tags: input.tags,
        })
        .execute(&mut db_conn)
        .await?;

    let inserted: db::Task = tasks.find(new_task_id.clone()).first(&mut db_conn).await?;

    Ok(CreatedResource {
        location: format!("/api/task/{new_task_id}"),
        resource: Resource::new(inserted.into()).with_links([
            ("mark", format!("/api/task/{new_task_id}/checkmark")),
            ("self", format!("/api/task/{new_task_id}")),
        ]),
    })
}
