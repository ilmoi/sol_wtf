#!/usr/bin/env bash
set -x
set -eo pipefail
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=dbpw}"
DB_NAME="${POSTGRES_DB:=solwtf}"
DB_PORT="${POSTGRES_PORT:=5432}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
fi

# wait for db to be ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# create db if not there / run migrations if have any available
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"

# to migrate the database (in terminal):
# 1) export DATABASE_URL=postgres://postgres:dbpw@localhost:5432/solwtf
# 2) sqlx migrate add file_to_describe_what_migration_is_doing
# 3) go into the above file and edit it wiith raw sql
# 4) sqlx migrate run OR SKIP_DOCKER=1 ./scripts/init_db.sh