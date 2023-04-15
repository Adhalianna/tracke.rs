// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (task_id) {
        task_id -> Uuid,
        tracker_id -> Uuid,
        completed_at -> Nullable<Timestamp>,
        title -> Varchar,
        description -> Nullable<Text>,
        time_estimate -> Nullable<Int8>,
        soft_deadline -> Nullable<Timestamp>,
        hard_deadline -> Nullable<Timestamp>,
        tags -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    trackers (tracker_id) {
        tracker_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        is_default -> Bool,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        email -> Bpchar,
        password -> Bytea,
    }
}

diesel::joinable!(tasks -> trackers (tracker_id));
diesel::joinable!(trackers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    trackers,
    users,
);
