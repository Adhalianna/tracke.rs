# Tooling

## Back-end Tech Stack
- Docker
- Rust (at 1 week trial period)
    - [Diesel](http://diesel.rs/) - an ORM
    - [Axum](https://docs.rs/axum) - thin web framework
    - [Utoipa](https://docs.rs/utoipa) - documentation generation
- PostgreSQL
- Discord API
- Swagger

## Required System Libraries
Diesel which is being used as an ORM for the project requires that 
libpq (postgres libraries) is present on the system. While this is not
a problem in the Docker environment, developing localy may require
installing the library first.

## Running the project with features for local developement
Execute `cargo dev` instead of `cargo run` to enable extra features which
make local development (without Docker) easier.

## Managing the database with migrations.
Migrations are managed by a [cli tool which comes with Diesel](https://github.com/diesel-rs/diesel/tree/master/diesel_cli).
Using the _up_ and _down_ pattern of migrations allows more easily
reproducible state of the database. The tool is introduced in the ["Getting
Started"](http://diesel.rs/guides/getting-started#installing-diesel-cli)
tutorial for Diesel ORM. Besides running the migrations it can also read the
state of the database and encode in Rust it's schema so that it is available
from the code.
