# Back-end Tech Stack
- Docker
- Rust (at 1 week trial period)
- PostgreSQL
- Discord API
- Swagger

# Required System Libraries
Diesel which is being used as an ORM for the project requires that 
libpq (postgres libraries) are present on the system. While this is not
a problem in the Docker environment, developing localy may require
installing the library first.

# Running the project for local development
Execute `cargo dev` instead of `cargo run` to enable extra features which
make local development (without Docker) easier.

# Running in Docker
