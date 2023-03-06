FROM rust:slim as chef
COPY . .
RUN apt-get update && apt-get install lld clang -y
RUN cargo install cargo-chef --locked --no-track
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Build our project
RUN cargo build --release --bin back-end \
    && strip /app/target/release/back-end

FROM debian:bullseye-slim AS runtime
WORKDIR .
RUN apt-get update \
    && apt-get install openssl ca-certificates libpq-dev -y
COPY --from=builder /app/target/release/back-end back-end
ENV DEPLOYMENT_ENVIRONMENT release
ENTRYPOINT ["./back-end"]