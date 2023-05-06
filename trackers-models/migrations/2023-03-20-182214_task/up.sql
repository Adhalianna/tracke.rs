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
  refresh_token varchar(1024) not null default null,
  started_at timestamp with time zone not null default now(),
  valid_until timestamp with time zone not null default now() + interval '30 minutes'
);
CREATE INDEX sessions_user_id_idx ON sessions USING HASH (user_id);
CREATE INDEX sessions_access_token_idx ON sessions USING HASH (access_token);

CREATE TABLE trackers(
  tracker_id uuid not null primary key,
  user_id uuid not null references users,
  name varchar(255) not null,
  is_default bool not null default false
);
CREATE INDEX trackers_tracker_id_idx ON trackers USING HASH (tracker_id);
CREATE INDEX trackers_user_id_idx ON trackers USING HASH (user_id);

CREATE TABLE tasks(
  task_id uuid not null primary key,
  tracker_id uuid not null references trackers,
  completed_at timestamp with time zone null,
  title varchar(255) not null,
  description text null,
  time_estimate bigint null, -- storing the number of seconds
  soft_deadline timestamp with time zone null,
  hard_deadline timestamp with time zone null,
  tags text[] null
);
CREATE INDEX tasks_task_id_idx ON tasks (task_id); -- B-Tree, uuid7 is sortable and we want to use it to paginate
CREATE INDEX tasks_tracker_id_idx ON tasks USING HASH (tracker_id);