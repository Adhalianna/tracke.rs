// @generated automatically by Diesel CLI.

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
        refresh_token -> Varchar,
        started_at -> Timestamptz,
        valid_until -> Timestamptz,
    }
}

diesel::table! {
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

diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(tasks -> trackers (tracker_id));
diesel::joinable!(trackers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    registration_requests,
    sessions,
    tasks,
    trackers,
    users,
);
