-- This file should undo anything in `up.sql`
DROP INDEX users_user_id_idx;
DROP INDEX users_email_idx;

DROP INDEX registration_requests_email_idx;

DROP INDEX sessions_user_id_idx;
DROP INDEX sessions_access_token_idx;

DROP INDEX trackers_tracker_id_idx;
DROP INDEX trackers_user_id_idx;

DROP INDEX tasks_task_id_idx;
DROP INDEX tasks_tracker_id_idx;

DROP INDEX authorised_clients_credentials_idx;
DROP INDEX authorised_clients_user_id_idx;

DROP INDEX views_view_id_idx;
DROP INDEX views_name_user_id_idx;
DROP INDEX views_user_id_idx;

DROP INDEX tracker_views_view_id_idx;
DROP INDEX tracker_views_tracker_id_idx;

DROP TABLE tracker_views;
DROP TABLE views;
DROP TABLE authorised_clients;
DROP TABLE tasks;
DROP TABLE trackers;
DROP TABLE registration_requests;
DROP TABLE sessions;
DROP TABLE users;

DROP TYPE view_kv_t;
DROP TYPE list_item_t;
