#!/bin/bash

# initialize the db and run migrations
SKIP_DOCKER=1 INSIDE_DOCKER_COMPOSE=1 /app/scripts/init_db.sh

# start the app
#cargo watch -x run | bunyan -o inspect
cargo run