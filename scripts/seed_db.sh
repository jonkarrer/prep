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


# Keep pinging MySQL until it's ready to accept commands
until mysql -h 127.0.0.1 -u root -pmy-secret-pw -P 3306 -D mysql -e 'SELECT 1'; do
  >&2 echo "MySQL is still unavailable - sleeping"
  sleep 1
done

# create migration with sqlx
sqlx database create --database-url mysql://root:my-secret-pw@localhost:3306/mysql
sqlx migrate run --source database/migrations --database-url mysql://root:my-secret-pw@localhost:3306/mysql