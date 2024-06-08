FROM lukemathwalker/cargo-chef:latest as chef
ARG DATABASE_URL
ENV DATABASE_URL $DATABASE_URL
WORKDIR /app

FROM chef AS planner
COPY . ./
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo install sqlx-cli
RUN sqlx migrate run --source ./app/persistence/migrations

RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/service-starter-rs ./app

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]
