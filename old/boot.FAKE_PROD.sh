#!/bin/bash

# wait for db to be ready
DB_PASSWORD=dbpw
DB_HOST=postgres
DB_USER=postgres
DB_PORT=5432
DB_NAME=solwtf

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# start the app
./backend