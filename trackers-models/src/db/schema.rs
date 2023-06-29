// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "list_item_t"))]
    pub struct ListItemT;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "view_kv_t"))]
    pub struct ViewKvT;
}

diesel::table! {
    authorised_clients (client_id) {
        user_id -> Uuid,
        name -> Varchar,
        website -> Varchar,
        client_id -> Varchar,
        client_secret -> Varchar,
    }
}

diesel::table! {
    registration_requests (email) {
        email -> Varchar,
        issued_at -> Timestamptz,
        valid_until -> Timestamptz,
        confirmation_code -> Bpchar,
        password -> Bytea,
    }
}

diesel::table! {
    sessions (access_token) {
        user_id -> Uuid,
        access_token -> Varchar,
        refresh_token -> Nullable<Varchar>,
        started_at -> Timestamptz,
        valid_until -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ListItemT;

    tasks (task_id) {
        task_id -> Uuid,
        tracker_id -> Uuid,
        completed_at -> Nullable<Timestamptz>,
        title -> Varchar,
        description -> Nullable<Text>,
        time_estimate -> Nullable<Int8>,
        soft_deadline -> Nullable<Timestamptz>,
        hard_deadline -> Nullable<Timestamptz>,
        tags -> Nullable<Array<Nullable<Text>>>,
        list -> Nullable<Array<Nullable<ListItemT>>>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ViewKvT;

    tracker_views (view_id, tracker_id) {
        view_id -> Uuid,
        tracker_id -> Uuid,
        name -> Nullable<Varchar>,
        keys_values -> Array<Nullable<ViewKvT>>,
    }
}

diesel::table! {
    trackers (tracker_id) {
        tracker_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        is_default -> Nullable<Bool>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        email -> Varchar,
        password -> Bytea,
    }
}

diesel::table! {
    views (user_id, name) {
        view_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
    }
}

diesel::joinable!(authorised_clients -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(tasks -> trackers (tracker_id));
diesel::joinable!(tracker_views -> trackers (tracker_id));
diesel::joinable!(trackers -> users (user_id));
diesel::joinable!(views -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    authorised_clients,
    registration_requests,
    sessions,
    tasks,
    tracker_views,
    trackers,
    users,
    views,
);
