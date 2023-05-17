use crate::prelude::*;
use models::{db, Task, TaskInput, Tracker, TrackerPatch, TrackerReplace};

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/tracker/:tracker_id",
            routing::get(get_one_tracker)
                .put(replace_tracker)
                .delete(delete_tracker)
                .patch(patch_tracker),
            |op| op.tag("Task Management"),
        )
        .api_route_with(
            "/tracker/:tracker_id/tasks",
            routing::get(get_trackers_tasks).post(post_to_tracker_a_task),
            |op| op.tag("Task Management"),
        )
        .layer(crate::auth::layer::authorizer().jwt_layer(crate::auth::layer::authority().clone()))
}

async fn get_one_tracker(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(tracker_id): axum::extract::Path<Base62Uuid>,
) -> Result<Resource<Tracker>, ApiError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::trackers::dsl::trackers;

    let the_tracker: Tracker = trackers
        // .filter(db_schema::trackers::user_id.eq(user_id.0))
        .find(tracker_id.clone())
        .get_result(&mut db_conn)
        .await?;

    if the_tracker.user_id != user_id.0 {
        Err(ForbiddenError::default().with_msg("no access to the selected task tracker"))?;
    }

    Ok(Resource::new(the_tracker)
        .with_links([("tasks", format!("/api/tracker/{tracker_id}/tasks"))]))
}

async fn patch_tracker(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(tracker_id): axum::extract::Path<Base62Uuid>,
    json: JsonExtract<TrackerPatch>,
) -> Result<ModifiedResource<Tracker>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let input = json.data;

    if let Some(json_tracker_id) = &input.tracker_id {
        if tracker_id != *json_tracker_id {
            Err(ConflictError::default().with_msg(
                "tracker id provided in path parameters and body fields are mismatching",
            ))?;
        }
    }

    // Check if tracker actually existed before update
    let res = db_schema::trackers::table
        .filter(db_schema::trackers::user_id.eq(&user_id.0))
        .find(&tracker_id)
        .execute(&mut db_conn)
        .await?;
    if res < 1 {
        let user_email: String = db_schema::users::table
            .find(&user_id.0)
            .select(db_schema::users::email)
            .get_result(&mut db_conn)
            .await?;

        Err(ConflictError::default()
            .with_msg("no such tracker exists, create one with a POST request")
            .with_docs()
            .with_links([("new tracker", format!("/api/user/{user_email}/trackers"))]))?;
    }

    let tracker: Tracker = diesel::update(db_schema::trackers::table)
        .filter(db_schema::trackers::tracker_id.eq(&tracker_id))
        .set(input)
        .get_result::<Tracker>(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(tracker).with_links([
            ("tasks", format!("/api/tracker/{tracker_id}/tasks")),
            ("documentation", format!("/doc")),
        ]),
    })
}

async fn replace_tracker(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(tracker_id): axum::extract::Path<Base62Uuid>,
    json: JsonExtract<Tracker>,
) -> Result<ModifiedResource<Tracker>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let input = json.data;

    if tracker_id != input.tracker_id {
        Err(ConflictError::default()
            .with_msg("tracker id provided in path parameters and body fields are mismatching"))?;
    }

    // Check if tracker actually existed before update
    let res = db_schema::trackers::table
        .filter(db_schema::trackers::user_id.eq(&user_id.0))
        .find(&tracker_id)
        .execute(&mut db_conn)
        .await?;
    if res < 1 {
        let user_email: String = db_schema::users::table
            .find(&user_id.0)
            .select(db_schema::users::email)
            .get_result(&mut db_conn)
            .await?;

        Err(ConflictError::default()
            .with_msg("no such tracker exists, create one with a POST request")
            .with_docs()
            .with_links([("new tracker", format!("/api/user/{user_email}/trackers"))]))?;
    }

    let tracker: Tracker = diesel::update(db_schema::trackers::table)
        .filter(db_schema::trackers::tracker_id.eq(&tracker_id))
        .set(TrackerReplace::from(input))
        .get_result::<Tracker>(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(tracker).with_links([
            ("tasks", format!("/api/tracker/{tracker_id}/tasks")),
            ("documentation", format!("/doc")),
        ]),
    })
}

async fn delete_tracker(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(tracker_id): axum::extract::Path<Base62Uuid>,
) -> Result<DeletedResource, ApiError> {
    let mut db_conn = state.db.get().await?;
    use db_schema::trackers::{columns, dsl::trackers};

    // TODO: use a transaction here

    let to_delete_search_res: diesel::result::QueryResult<Tracker> = trackers
        .filter(
            columns::tracker_id
                .eq(&tracker_id)
                .and(columns::user_id.eq(&user_id.0))
                .and(columns::is_default.eq(models::types::NullOrTrue::Null)),
        )
        .first(&mut db_conn)
        .await;

    let to_delete: Tracker = match to_delete_search_res {
        Ok(tracker) => tracker,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let exist_not_default = trackers
                    .filter(
                        columns::tracker_id
                            .eq(&tracker_id)
                            .and(columns::user_id.eq(&user_id.0)),
                    )
                    .execute(&mut db_conn)
                    .await?;
                if exist_not_default > 1 {
                    return Err(ForbiddenError::default().with_docs().with_msg("the specified task tracker is considered the default task tracker for the user and as such it cannot be removed").into());
                } else {
                    return Err(err.into());
                }
            }
            _ => return Err(err.into()),
        },
    };

    // delete all the tasks that belonged to that tracker
    diesel::delete(db_schema::tasks::table)
        .filter(db_schema::tasks::tracker_id.eq(&to_delete.tracker_id))
        .execute(&mut db_conn)
        .await?;

    let affected = diesel::delete(trackers)
        .filter(columns::tracker_id.eq(&to_delete.tracker_id))
        .execute(&mut db_conn)
        .await?;

    if affected < 1 {
        Err(ServerError::from(anyhow::anyhow!(
            "failed to delete anything despite successfull query execution"
        )))?;
    }

    Ok(DeletedResource {
        ..Default::default()
    })
}

async fn get_trackers_tasks(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    query: Option<QsQuery<crate::query_param::TasksQuery>>,
    axum::extract::Path(the_tracker_id): axum::extract::Path<Base62Uuid>,
) -> Result<Resource<Vec<Task>>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let mut tasks_query = db_schema::trackers::table
        .filter(
            db_schema::trackers::columns::tracker_id
                .eq(the_tracker_id)
                .and(db_schema::trackers::columns::user_id.eq(user_id.0)),
        )
        .inner_join(db_schema::tasks::table)
        .select(db_schema::tasks::all_columns)
        .into_boxed();

    if let Some(query) = query {
        tasks_query = tasks_query.filter(query.into_join_filters());
    }

    let trackers_tasks: Vec<db::Task> = tasks_query.load(&mut db_conn).await?;

    Ok(Resource::new({
        trackers_tasks.into_iter().map(|t| t.into()).collect()
    }))
}

async fn post_to_tracker_a_task(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(the_tracker_id): axum::extract::Path<Base62Uuid>,
    json: JsonExtract<TaskInput>,
) -> Result<CreatedResource<Task>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let input = json.data;

    if let Some(json_tracker_id) = &input.tracker_id {
        if the_tracker_id != *json_tracker_id {
            Err(ConflictError::default().with_msg(
                "tracker id given in the path does not match with the tracker id provided in the body",
            ).with_docs())?;
        }
    }

    use db_schema::tasks::dsl::*;
    let new_task_id = input.task_id.unwrap_or(uuid::Uuid::now_v7().into());

    // Check if the tracker is owned by the user
    let res = db_schema::trackers::table
        .filter(
            db_schema::trackers::tracker_id
                .eq(&the_tracker_id)
                .and(db_schema::trackers::user_id.eq(user_id.0)),
        )
        .execute(&mut db_conn)
        .await?;
    if res < 1 {
        Err(ForbiddenError::default().with_msg("no access to the selected tracker"))?;
    }

    diesel::insert_into(tasks)
        .values(db::Task {
            task_id: new_task_id.clone(),
            tracker_id: the_tracker_id,
            title: input.title,
            description: input.description,
            completed_at: {
                if input.checkmarked {
                    Some(chrono::Utc::now())
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
