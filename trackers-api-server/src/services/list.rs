use axum::extract::Path;
use models::types::ListItems;

use crate::{
    auth::{scope::UserIdScope, UserClaims, VariableScope},
    prelude::*,
};

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
        .api_route_with(
            "/task/:task_id/list/item/:idx",
            routing::get(get_list_item)
                .delete(delete_list_item)
                .put(replace_list_item)
                .post(create_list_item_with_idx),
            |op| op.tag("Task Management"),
        )
        .api_route_with(
            "/task/:task_id/list/item/:idx/checkmark",
            routing::delete(remove_item_checkmark)
                .put(mark_item_done)
                .post(mark_item_done),
            |op| op.tag("Task Management"),
        )
        .api_route_with(
            "/task/:task_id/list/items",
            routing::post(create_list_item),
            |op| op.tag("Task Management"),
        )
        .layer(crate::auth::layer::authorizer().jwt_layer(crate::auth::layer::authority().clone()))
}

async fn modify_item_checkmark(
    set_value: bool,
    db_conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
    task_id: uuid::Uuid,
    item_idx: usize,
    user_id: models::types::Uuid,
) -> Result<ModifiedResource<models::types::ListItem>, ApiError> {
    let Some(mut list) = try_get_securely_just_the_list(db_conn, task_id, user_id).await? else {
        return Err(NotFoundError::default()
            .with_docs()
            .with_msg("no list found for selected task")
            .with_links([
                ("create", format!("/api/task/{task_id}/list")),
                ("task", format!("/api/task/{task_id}")),
            ])
            .into())
    };

    let Some(item) = list.0.get_mut(item_idx - 1) else {
        return Err(NotFoundError::default()
            .with_docs()
            .with_msg("no item found under provided index number")
            .with_links([
                ("create", format!("/api/task/{task_id}/list/item/{item_idx}")),
                ("list", format!("/api/task/{task_id}/list")),
                ("task", format!("/api/task/{task_id}")),
            ])
            .into())
    };

    item.is_completed = set_value;

    diesel::update(db_schema::tasks::table)
        .filter(db_schema::tasks::task_id.eq(task_id))
        .set(db_schema::tasks::list.eq(&list))
        .execute(db_conn)
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(list.0.remove(item_idx - 1)),
    })
}

async fn remove_item_checkmark(
    State(state): State<AppState>,
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path((task_id, item_idx)): Path<(Base62Uuid, usize)>,
) -> Result<ModifiedResource<models::types::ListItem>, ApiError> {
    let mut db_conn = state.db.get().await?;
    modify_item_checkmark(false, &mut db_conn, task_id.into(), item_idx, user_id.0).await
}

async fn mark_item_done(
    State(state): State<AppState>,
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path((task_id, item_idx)): Path<(Base62Uuid, usize)>,
) -> Result<ModifiedResource<models::types::ListItem>, ApiError> {
    let mut db_conn = state.db.get().await?;
    modify_item_checkmark(true, &mut db_conn, task_id.into(), item_idx, user_id.0).await
}

async fn create_list_item(
    State(state): State<AppState>,
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path(task_id): Path<Base62Uuid>,
    json: JsonExtract<models::types::ListItem>,
) -> Result<ModifiedResource<models::types::ListItems>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let new_item = json.extract();

    let maybe_list =
        try_get_securely_just_the_list(&mut db_conn, task_id.clone().into(), user_id.0).await?;

    // create a new list with the item if there was none before or add
    let list = match maybe_list {
        Some(mut list) => {
            list.0.push(new_item);
            list
        }
        None => ListItems(vec![new_item]),
    };

    diesel::update(db_schema::tasks::table)
        .filter(db_schema::tasks::task_id.eq(task_id))
        .set(db_schema::tasks::list.eq(&list))
        .execute(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(list),
    })
}

async fn create_list_item_with_idx(
    State(state): State<AppState>,
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path((task_id, item_idx)): Path<(Base62Uuid, usize)>,
    json: JsonExtract<models::types::ListItem>,
) -> Result<ModifiedResource<models::types::ListItems>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let new_item = json.extract();

    let Some(mut list) = try_get_securely_just_the_list(&mut db_conn, task_id.clone().into(), user_id.0).await? else {
        return Err(NotFoundError::default()
            .with_docs()
            .with_msg("no list found for selected task")
            .with_links([
                ("create", format!("/api/task/{task_id}/list")),
                ("task", format!("/api/task/{task_id}")),
            ])
            .into())
    };

    let list_inner = &mut list.0;

    if item_idx < list_inner.len() {
        return Err(ConflictError::default()
            .with_docs()
            .with_msg("An item with this index number already exists")
            .with_links([
                (
                    "delete",
                    format!("/api/task/{task_id}/list/item/{item_idx}"),
                ),
                (
                    "replace",
                    format!("/api/task/{task_id}/list/item/{item_idx}"),
                ),
                ("full list", format!("/api/task/{task_id}/list")),
            ])
            .into());
    } else {
        list_inner.push(new_item);

        diesel::update(db_schema::tasks::table)
            .filter(db_schema::tasks::task_id.eq(task_id))
            .set(db_schema::tasks::list.eq(&list))
            .execute(&mut db_conn)
            .await?;

        Ok(ModifiedResource {
            location: None,
            resource: Resource::new(list),
        })
    }
}

async fn replace_list_item(
    State(state): State<AppState>,
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path((task_id, item_idx)): Path<(Base62Uuid, usize)>,
    json: JsonExtract<models::types::ListItem>,
) -> Result<ModifiedResource<models::types::ListItems>, ApiError> {
    let mut db_conn = state.db.get().await?;
    let new_item = json.extract();

    let Some(mut list) = try_get_securely_just_the_list(&mut db_conn, task_id.clone().into(), user_id.0).await? else {
        return Err(NotFoundError::default()
            .with_docs()
            .with_msg("no list found for selected task")
            .with_links([
                ("create", format!("/api/task/{task_id}/list")),
                ("task", format!("/api/task/{task_id}")),
            ])
            .into())
    };

    let list_inner = &mut list.0;
    match list_inner.get_mut(item_idx - 1) {
        Some(item) => {
            *item = new_item;
        }
        None => {
            if item_idx == list_inner.len() {
                // allow creation of new items through PUT if the index is the next index in the list
                list_inner.push(new_item);
            } else {
                // invalid index
                return Err(ConflictError::default()
                    .with_docs()
                    .with_msg("PUT cannot be used to reorder items")
                    .with_links([
                        ("create", format!("/api/task/{task_id}/list/items")),
                        ("full list", format!("/api/task/{task_id}/list")),
                    ])
                    .into());
            }
        }
    }

    diesel::update(db_schema::tasks::table)
        .filter(db_schema::tasks::task_id.eq(task_id))
        .set(db_schema::tasks::list.eq(&list))
        .execute(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(list),
    })
}

async fn delete_list_item(
    State(state): State<AppState>,
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path((task_id, item_idx)): Path<(Base62Uuid, usize)>,
) -> Result<ModifiedResource<models::types::ListItems>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let Some(mut list) = try_get_securely_just_the_list(&mut db_conn, task_id.clone().into(), user_id.0).await? else {
        return Err(NotFoundError::default()
            .with_docs()
            .with_msg("no list found for selected task")
            .with_links([
                ("create", format!("/api/task/{task_id}/list")),
                ("task", format!("/api/task/{task_id}")),
            ])
            .into())
    };

    // modify the whole list by removing the item
    let list_inner = &mut list.0;
    if list_inner.get(item_idx - 1).is_none() {
        return Err(NotFoundError::default()
            .with_docs()
            .with_msg("no item with provided index found")
            .with_links([("create", format!("/api/task/{task_id}/list"))])
            .into());
    };
    list_inner.remove(item_idx - 1);

    // save in the database
    diesel::update(db_schema::tasks::table)
        .filter(db_schema::tasks::task_id.eq(task_id))
        .set(db_schema::tasks::list.eq(&list))
        .execute(&mut db_conn)
        .await?;

    Ok(ModifiedResource {
        location: None,
        resource: Resource::new(list),
    })
}

async fn get_list_item(
    State(state): State<AppState>,
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path((task_id, item_idx)): Path<(Base62Uuid, usize)>,
) -> Result<Resource<models::types::ListItem>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let Some(list) = try_get_securely_just_the_list(&mut db_conn, task_id.clone().into(), user_id.0).await? else {
        return Err(NotFoundError::default()
            .with_docs()
            .with_msg("no list found for selected task")
            .with_links([
                ("create", format!("/api/task/{task_id}/list")),
                ("task", format!("/api/task/{task_id}")),
            ])
            .into())
    };

    let list = list.0;
    match list.get(item_idx - 1) {
        Some(item) => {
            let resource = Resource::new(item.to_owned());
            //TODO: add checkmark links
            Ok(resource)
        }
        None => Err(NotFoundError::default()
            .with_docs()
            .with_msg("no item with provided index found")
            .with_links([("create", format!("/api/task/{task_id}/list"))])
            .into()),
    }
}

async fn try_get_securely_just_the_list(
    db_conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
    task_id: uuid::Uuid,
    user_id: models::types::Uuid,
) -> Result<Option<ListItems>, ApiError> {
    let (list, task_user_id) = db_schema::tasks::table
        .inner_join(db_schema::trackers::table)
        .filter(db_schema::tasks::task_id.eq(&task_id))
        .select((db_schema::tasks::list, db_schema::trackers::user_id))
        .first::<(Option<models::types::ListItems>, models::types::Uuid)>(db_conn)
        .await?;

    if task_user_id != user_id {
        Err(ForbiddenError::default()
            .with_docs()
            .with_msg("no access to the selected task"))?;
    }

    Ok(list)
}

async fn get_just_list(
    State(state): State<AppState>,
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path(task_id): Path<Base62Uuid>,
) -> Result<Resource<models::types::ListItems>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let list =
        try_get_securely_just_the_list(&mut db_conn, task_id.clone().into(), user_id.0).await?;

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
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path(task_id): Path<Base62Uuid>,
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
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path(task_id): Path<Base62Uuid>,
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
    VariableScope(user_id): VariableScope<UserIdScope, UserClaims>,
    Path(task_id): Path<Base62Uuid>,
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
