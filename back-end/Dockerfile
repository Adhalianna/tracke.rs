FROM lukemathwalker/cargo-chef:latest-rust-alpine as chef
WORKDIR /app
RUN apk update && apk add --no-cache lld clang musl-dev

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
# Build our project
RUN cargo build --release --target x86_64-unknown-linux-musl --bin back-end \
    && strip /app/target/x86_64-unknown-linux-musl/release/back-end

FROM alpine AS runtime
WORKDIR .
RUN apk update \
    && apk add --no-cache openssl ca-certificates libpq
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/back-end back-end
ENV DEPLOYMENT_ENVIRONMENT release
ENTRYPOINT ["./back-end"]