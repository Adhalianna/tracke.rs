use models::types::ListItems;

use crate::prelude::*;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/task/:task_id/list",
            routing::get(get_just_list)
                .post(add_list)
                .put(replace_list)
                .delete(delete_list),
            |op| op.tag("Task Management"),
        )
        // .api_route_with("/task/:task_id/list/:idx", routing::get(handler), |op| {
        //     op.tag("Task Management")
        // })
        .layer(crate::auth::layer::authorizer().jwt_layer(crate::auth::layer::authority().clone()))
}

// async fn get_list_item()

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
        .filter(db_schema::tasks::task_id.eq(&task_id))
        .select((db_schema::tasks::list, db_schema::trackers::user_id))
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
            .with_links([
                ("create", format!("/api/task/{task_id}/list")),
                ("task", format!("/api/task/{task_id}")),
            ])
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
) -> Result<CreatedResource<models::types::ListItems>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let new_list = json.extract();

    // Check the list like in the GET request:
    let (list, task_user_id) = db_schema::tasks::table
        .inner_join(db_schema::trackers::table)
        .filter(db_schema::tasks::task_id.eq(&task_id))
        .select((db_schema::tasks::list, db_schema::trackers::user_id))
        .first::<(Option<models::types::ListItems>, models::types::Uuid)>(&mut db_conn)
        .await?;

    if task_user_id != user_id.0 {
        Err(ForbiddenError::default()
            .with_docs()
            .with_msg("no access to the selected task"))?;
    }

    if list.is_some() {
        Err(ConflictError::default()
            .with_msg("list already present")
            .with_links([
                ("put", format!("/api/task/{task_id}/list")),
                ("delete", format!("/api/task/{task_id}/list")),
            ])
            .with_docs())?;
    }

    // Run the task update when everything checks out
    diesel::update(db_schema::tasks::table)
        .filter(db_schema::tasks::task_id.eq(&task_id))
        .set(db_schema::tasks::list.eq(&new_list))
        .execute(&mut db_conn)
        .await?;

    Ok(CreatedResource {
        location: format!("/api/task/{task_id}/list"),
        resource: Resource::new(new_list),
    })
}

async fn replace_list(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
    json: JsonExtract<Option<models::types::ListItems>>,
) -> Result<DeletedOrModified<ListItems>, ApiError> {
    use DeletedOrModified::*;
    let mut db_conn = state.db.get().await?;
    let maybe_list = json.extract();

    let owned_by: models::types::Uuid = db_schema::tasks::table
        .inner_join(db_schema::trackers::table)
        .select(db_schema::trackers::user_id)
        .filter(db_schema::tasks::task_id.eq(&task_id))
        .get_result(&mut db_conn)
        .await?;
    if owned_by != user_id.0 {
        Err(ForbiddenError::default().with_msg("no access to the selected task"))?;
    }

    if let Some(list) = maybe_list {
        let _: usize = diesel::update(db_schema::tasks::table)
            .filter(db_schema::tasks::task_id.eq(&task_id))
            .set(db_schema::tasks::list.eq(&list))
            .execute(&mut db_conn)
            .await?;
        Ok(Modified(ModifiedResource {
            location: None,
            resource: Resource::new(list),
        }))
    } else {
        let _: usize = diesel::update(db_schema::tasks::table)
            .filter(db_schema::tasks::task_id.eq(&task_id))
            .set(db_schema::tasks::list.eq(Option::<ListItems>::None))
            .execute(&mut db_conn)
            .await?;

        Ok(Deleted(DeletedResource::default()))
    }
}

async fn delete_list(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(task_id): axum::extract::Path<Base62Uuid>,
) -> Result<DeletedResource, ApiError> {
    let mut db_conn = state.db.get().await?;

    let owned_by: models::types::Uuid = db_schema::tasks::table
        .inner_join(db_schema::trackers::table)
        .select(db_schema::trackers::user_id)
        .filter(db_schema::tasks::task_id.eq(&task_id))
        .get_result(&mut db_conn)
        .await?;
    if owned_by != user_id.0 {
        Err(ForbiddenError::default().with_msg("no access to the selected task"))?;
    }

    let _: usize = diesel::update(db_schema::tasks::table)
        .filter(db_schema::tasks::task_id.eq(&task_id))
        .set(db_schema::tasks::list.eq(Option::<ListItems>::None))
        .execute(&mut db_conn)
        .await?;

    Ok(DeletedResource::default())
}

#[derive(aide::OperationIo)]
pub enum DeletedOrModified<T> {
    Deleted(DeletedResource),
    Modified(ModifiedResource<T>),
}

impl<T: serde::Serialize> axum::response::IntoResponse for DeletedOrModified<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            DeletedOrModified::Deleted(del) => del.into_response(),
            DeletedOrModified::Modified(modi) => modi.into_response(),
        }
    }
}
