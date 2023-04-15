# tracke.rs

## Project Description

_tracke.rs_ - a hackable task management tool (an API) that aims to be reusable
for personal use in various contexts so that you never have to look for another
to-dos tracking tool again.

### Design goals

- __Hackable__ - the task manager is accessible through an open and documented
API.
- __Personal__ - while sharing and collaboration are possible the presentation
is focused on personal task management.
- __Flexible and task-centric__ - any productivity increasing technique should
be possible to reproduce with flexible tasks as the primary unit of knowledge.

### CS Web Application Technologies project goals

It is quite possible that because of limited time only a part of the design
goals will be realized by the deadline. It is a conceptually simple
application that could see growth in many directions.

### Technology stack
- Rust - language used on both front and back end for its stability and
ergonomic libraries
- axum - web server framework
- aliri - auth library
- PostgreSQL - database
- Diesel (async) - ORM
- aide - OAS generation from code
