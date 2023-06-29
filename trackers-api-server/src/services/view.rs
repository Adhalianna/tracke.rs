use models::db::TmpViewVec;

use crate::prelude::*;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/user/:email/views",
        routing::get(get_all_views).post(create_view),
        |op| op.tag("Tracker Views"),
    )
}

async fn get_single_view(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path((email, view_name)): axum::extract::Path<(
        models::types::Email,
        models::types::String<256>,
    )>,
) -> Result<Resource<models::View>, ApiError> {
    todo!()
}

async fn get_all_views(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(email): axum::extract::Path<models::types::Email>,
) -> Result<Resource<Vec<models::View>>, ApiError> {
    let mut db_conn = state.db.get().await?;

    use diesel::JoinOnDsl;

    todo!()

    // let views: Vec<(models::db::View, models::db::TrackerView, String)> = db_schema::views::table
    //     .inner_join(db_schema::users::table)
    //     .filter(
    //         db_schema::users::email
    //             .eq(email)
    //             .and(db_schema::users::user_id.eq(user_id.0)),
    //     )
    //     .inner_join(db_schema::tracker_views::table)
    //     .inner_join(
    //         db_schema::trackers::table
    //             .on(db_schema::trackers::tracker_id.eq(db_schema::tracker_views::tracker_id)),
    //     )
    //     .select((
    //         db_schema::views::all_columns,
    //         db_schema::tracker_views::all_columns,
    //         db_schema::trackers::name,
    //     ))
    //     .get_results(&mut db_conn)
    //     .await?;

    // let views: Vec<models::core::View> = TmpViewVec::from(views).into();

    // Ok(Resource::new(views).with_links([("documentation", "doc".into()), ("create", format!(""))]))
}

async fn create_view(
    State(state): State<AppState>,
    crate::auth::VariableScope(user_id): crate::auth::VariableScope<
        crate::auth::scope::UserIdScope,
        crate::auth::UserClaims,
    >,
    axum::extract::Path(email): axum::extract::Path<models::types::Email>, //todo: do sth about it
    json: JsonExtract<models::CreateView>,
) -> Result<CreatedResource<models::View>, ApiError> {
    let mut db_conn = state.db.get().await?;

    let view = json.extract();

    if view.user_id != user_id.0 {
        return Err(ForbiddenError::default()
            .with_docs()
            .with_msg("can only create views for the user themselves")
            .into());
    }

    let created_view = models::View {
        view_id: models::types::Uuid::new(),
        user_id: view.user_id,
        name: view.name,
        trackers: view.trackers,
    };
    let (view, tracker_views) = created_view.clone().into();

    diesel::insert_into(db_schema::views::table)
        .values(view)
        .execute(&mut db_conn)
        .await?;
    diesel::insert_into(db_schema::tracker_views::table)
        .values(tracker_views)
        .execute(&mut db_conn)
        .await?;

    Ok(CreatedResource {
        resource: Resource::new(created_view),
        location: todo!(),
    })
}
