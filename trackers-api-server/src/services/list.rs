use crate::prelude::*;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with("task/:task_id/list", routing::get(get_just_list), |op| {
            op.tag("Task Management")
        })
        .layer(crate::auth::layer::authorizer().jwt_layer(crate::auth::layer::authority().clone()))
}

async fn get_just_list(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<Resource<models::types::ListItems>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let (list, task_user_id) = db_schema::tasks::table
        .inner_join(db_schema::trackers::table)
        .filter(db_schema::tasks::task_id.eq(task_id))
        .select((db_schema::tasks::list, db_schema::trackers::tracker_id))
        .first::<(Option<models::types::ListItems>, models::types::Uuid)>(&mut db_conn)
        .await?;

    if task_user_id != user_id.0 {
        Err(ForbiddenError::default()
            .with_docs()
            .with_msg("no access to the selected task"))?;
    }

    match list {
        Some(list) => {
            let resource = Resource::new(list);
            //TODO: add checkmark links
            Ok(resource)
        }
        None => Err(NotFoundError::default()
            .with_docs()
            .with_msg("no list found for selected task")
            .into()),
    }
}

async fn add_list(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
    json: JsonExtract<models::types::ListItems>,
) -> Result<Resource<models::types::ListItems>, ApiError> {
    todo!()
}
