#!/bin/bash

# get details to connect to db
DB_PASSWORD=$(grep -A2 'database:' ./config/prod_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')
DB_HOST=$(grep -A1 'database:' ./config/prod_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')
DB_USER=$(grep -A2 'database:' ./config/base_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')
DB_PORT=$(grep -A1 'database:' ./config/base_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')
DB_NAME=$(grep -A3 'database:' ./config/base_config.yml | tail -n1 | awk '{ print $2}' | tr -d '"')

# wait for db to be ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# start the app
./backend