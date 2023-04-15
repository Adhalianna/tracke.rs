-- Your SQL goes here
CREATE TABLE users(
  user_id uuid not null primary key,
  email char(320) not null unique,
  password varchar(255) not null
);

CREATE TABLE groups(
  group_id uuid not null primary key,
  user_id uuid not null references users,
  name varchar(255) not null unique
);

CREATE TABLE tasks(
  task_id uuid not null primary key,
  user_id uuid null references users,
  group_id uuid null references groups,
  title varchar(255) not null,
  description text null,
  time_estimate bigint null, -- storing the number of seconds
  soft_deadline timestamp null,
  hard_deadline timestamp null,
  tags text[] null
);