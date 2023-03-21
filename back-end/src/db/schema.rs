// @generated automatically by Diesel CLI.

diesel::table! {
    groups (group_id) {
        group_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    tasks (task_id) {
        task_id -> Uuid,
        user_id -> Nullable<Uuid>,
        group_id -> Nullable<Uuid>,
        title -> Varchar,
        description -> Nullable<Text>,
        time_estimate -> Nullable<Int8>,
        soft_deadline -> Nullable<Timestamp>,
        hard_deadline -> Nullable<Timestamp>,
        tags -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        email -> Bpchar,
        password -> Varchar,
    }
}

diesel::joinable!(groups -> users (user_id));
diesel::joinable!(tasks -> groups (group_id));
diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    tasks,
    users,
);
