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

# Copy over files needed to compile the rest of the image. (!) If any of these change, this step will have to be redone.
COPY .cargo .cargo
COPY src src
COPY .env .env
COPY Cargo.toml Cargo.toml
COPY sqlx-data.json sqlx-data.json

# Build our application, leveraging the cached deps!
ENV SQLX_OFFLINE true
RUN cargo build --release --bin backend

# ------------------------------------------------------------------------------ prepare final image
FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates postgresql vim bash \
    # Clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend backend

# header (image, deps, user, workdir)
# ------------------------------------------------------------------------------

# Copy the remaining files
COPY config config
COPY scripts scripts
COPY migrations migrations

RUN chmod +x app/scripts/boot.PROD.sh
RUN chmod +x app/scripts/db_init.sh

ENV APP_ENVIRONMENT prod

# ------------------------------------------------------------------------------
# footer (change user, run stuff)

#ports / expose / cmd / entrypoint - all done in docker-compose
