-- Your SQL goes here

CREATE TABLE users(
  user_id uuid not null primary key,
  email varchar(320) not null unique,
  password bytea not null
);
CREATE INDEX users_user_id_idx ON users USING HASH (user_id);
CREATE INDEX users_email_idx ON users USING HASH (user_id);

CREATE TABLE registration_requests(
  email varchar(320) not null primary key,
  issued_at timestamp with time zone not null default now(),
  valid_until timestamp with time zone not null default now() + interval '10 minutes',
  confirmation_code char(9) not null,
  password bytea not null
);
CREATE INDEX registration_requests_email_idx ON registration_requests USING HASH (email);

CREATE TABLE sessions(
  user_id uuid not null references users,
  access_token varchar(1024) not null primary key default null,
  refresh_token varchar(1024) null default null,
  started_at timestamp with time zone not null default now(),
  valid_until timestamp with time zone not null default now() + interval '30 minutes'
);
CREATE INDEX sessions_user_id_idx ON sessions USING HASH (user_id);
CREATE INDEX sessions_access_token_idx ON sessions USING HASH (access_token);

CREATE TABLE trackers(
  tracker_id uuid not null primary key,
  user_id uuid not null references users,
  name varchar(256) not null,
  is_default bool null default null,
  UNIQUE NULLS DISTINCT (user_id, is_default)  --this is important property which will be used to assert there's a single default for each user
);
CREATE INDEX trackers_tracker_id_idx ON trackers USING HASH (tracker_id);
CREATE INDEX trackers_user_id_idx ON trackers USING HASH (user_id);

CREATE TYPE list_item_t AS (
  item_content text,
  is_completed bool
);

CREATE TABLE tasks(
  task_id uuid not null primary key,
  tracker_id uuid not null references trackers,
  completed_at timestamp with time zone null,
  title varchar(256) not null,
  description text null,
  time_estimate bigint null, -- storing the number of seconds
  soft_deadline timestamp with time zone null,
  hard_deadline timestamp with time zone null,
  tags text[] null,
  list list_item_t[] null default null check (cardinality(list)>0 AND array_length(list, 1) <= 128 AND null != ANY(list))
);

CREATE INDEX tasks_task_id_idx ON tasks (task_id); -- B-Tree, uuid7 is sortable and we want to use it to paginate
CREATE INDEX tasks_tracker_id_idx ON tasks USING HASH (tracker_id);

CREATE TABLE authorised_clients(
  user_id uuid not null references users,
  name varchar(256) not null,
  website varchar not null,
  client_id varchar not null primary key,
  client_secret varchar not null unique
);

CREATE INDEX authorised_clients_credentials_idx ON authorised_clients (client_id, client_secret);
CREATE INDEX authorised_clients_user_id_idx ON authorised_clients USING HASH (user_id);

CREATE TABLE views(
  view_id uuid not null unique,
  user_id uuid not null references users,
  name varchar(256) not null,
  primary key (user_id, name)
);

CREATE TYPE view_kv_t AS (
  key varchar(64),
  value varchar(64)
);

CREATE INDEX views_view_id_idx ON views USING HASH(view_id);
CREATE INDEX views_name_user_id_idx ON views (user_id, name);
CREATE INDEX views_user_id_idx ON views USING HASH (user_id);

CREATE TABLE tracker_views(
  view_id uuid not null references views (view_id),
  tracker_id uuid not null references trackers (tracker_id),
  name varchar(256) null,
  keys_values view_kv_t[] not null default array[]::view_kv_t[] check (cardinality(keys_values)>=0 AND array_length(keys_values, 1) <= 128 AND null != ANY(keys_values)),
  primary key (view_id, tracker_id)
);

CREATE INDEX tracker_views_view_id_idx ON tracker_views USING HASH (view_id);
CREATE INDEX tracker_views_tracker_id_idx ON tracker_views USING HASH (tracker_id);