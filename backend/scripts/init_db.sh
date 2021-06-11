#!/usr/bin/env bash
set -x
set -eo pipefail

# get details to connect to db
DB_PASSWORD=$(grep -A2 'database:' ./config/dev_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')
DB_HOST=$(grep -A1 'database:' ./config/dev_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')
DB_USER=$(grep -A2 'database:' ./config/base_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')
DB_PORT=$(grep -A1 'database:' ./config/base_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')
DB_NAME=$(grep -A3 'database:' ./config/base_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')

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
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# create db if not there / run migrations if have any available
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"

# to migrate the database (in terminal):
# 1) export DATABASE_URL=postgres://postgres:dbpw@localhost:5432/solwtf
# 2) sqlx migrate add migration_name
# 3) go into the above file and edit it wiith raw sql
# 4) sqlx migrate run OR SKIP_DOCKER=1 ./scripts/init_db.sh