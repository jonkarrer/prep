#!/bin/bash
# print each command before it is executed
set -x
# stop on any error from any command in a pipeline. 
set -eo pipefail

# check for dependancies
if ! [ -x "$(command -v mysql)" ]; then
  echo >&2 "Error: mysql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  exit 1
fi

# set env vars
if [[ "${ENV_CONFIG}" == 'dev' ]]; then
  source .env
else
  source .env.prod
fi

# run docker command
if [[ -z ${SKIP_DOCKER} ]]; then
docker run \
    --detach \
    --name mysql \
    --env MYSQL_ROOT_PASSWORD=${DB_PASSWORD} \
    -p "${DB_PORT}":"${DB_PORT}" \
    mysql:latest;

    # Keep pinging MySQL until it's ready to accept commands
    until mysql -h 127.0.0.1 -u "${DB_USER}" -p"${DB_PASSWORD}" -P "${DB_PORT}" -D "${DB_NAME}" -e 'SELECT 1'; do
      >&2 echo "MySQL is still unavailable - sleeping"
      sleep 1
    done
fi

# create migration with sqlx
sqlx database create
sqlx migrate run --source database/migrations