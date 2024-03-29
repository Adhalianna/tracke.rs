# [tracke.rs](https://github.com/Adhalianna/tracke.rs)

## Project Description

___tracke.rs___ - a hackable task management tool (an API) that aims to be reusable
for personal use in various contexts so that you never have to look for another
to-dos tracking tool again.

The name is a reference to the common practice of using _.rs_ domains for
Rust-related projects. The `.rs` is a file extension used for Rust source files.

### Design goals

- __Hackable__ - the task manager is accessible through an open and documented
API.
- __Personal__ - while sharing and collaboration are possible the presentation
is focused on personal task management.
- __Flexible and task-centric__ - any productivity increasing technique should
be possible to reproduce with flexible tasks as the primary unit of knowledge.

### CS Web Application Technologies project goals

Because of limited time only a part of the design goals will be realized by the
deadline of WAT 2023 project. It is a conceptually simple application that could
see growth in many directions. The listed design goals are meant to set a 
direction for further development and instead the user stories that belong to the
[GitHub project](https://github.com/users/Adhalianna/projects/1) which is used for
tracking the WAT2023 project will be targeted.

### Technology stack
- Rust - language used on both front and back end for its stability and
ergonomic libraries
- axum - web server framework
- aliri - auth library
- PostgreSQL - database
- Diesel (async) - ORM
- aide - OAS generation from code
