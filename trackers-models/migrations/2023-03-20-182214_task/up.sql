-- Your SQL goes here
CREATE TABLE users(
  user_id uuid not null primary key,
  email char(320) not null unique,
  password bytea not null
);

CREATE TABLE trackers(
  tracker_id uuid not null primary key,
  user_id uuid not null references users,
  name varchar(255) not null,
  is_default bool not null default false
);

CREATE TABLE tasks(
  task_id uuid not null primary key,
  tracker_id uuid not null references trackers,
  completed_at timestamp null,
  title varchar(255) not null,
  description text null,
  time_estimate bigint null, -- storing the number of seconds
  soft_deadline timestamp null,
  hard_deadline timestamp null,
  tags text[] null
);