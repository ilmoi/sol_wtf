# ------------------------------------------------------------------------------ planning
FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

# ------------------------------------------------------------------------------ caching of deps
FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

# ------------------------------------------------------------------------------ build the app
FROM rust:1.50 AS builder
WORKDIR app
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
# Build our application, leveraging the cached deps!
ENV SQLX_OFFLINE true
RUN cargo build --release --bin backend

# ------------------------------------------------------------------------------ prepare final image
FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend backend
COPY config config
ENV APP_ENVIRONMENT prod
ENTRYPOINT ["./backend"]

#todo ok but how do you run migrations without slqx cli in prod?