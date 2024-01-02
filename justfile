# ---- Build ----
build-app:
    ./scripts/build.sh

teardown-app:
    docker compose down

# ---- Development ----
run-dev:
    cargo run --bin prep

# ---- DATABASE ----
echo-db-url:
    @source .env && echo "${DATABASE_URL}"

## init docker database instance and run migrations
init-db env_config:
    export ENV_CONFIG={{env_config}} && ./scripts/init_db.sh && cargo run --bin seeder

start-db:
    docker start mysql

stop-db:
    docker stop mysql

kill-db:
    docker kill mysql && docker rm -f mysql

## migrations
migrate-add file_name:
    @source .env \
    && sqlx migrate add --source database/migrations {{file_name}}

run-migration env_config:
    export ENV_CONFIG={{env_config}} \
    && export SKIP_DOCKER=true \
    && ./scripts/init_db.sh

# ---- Tests ----------------------------------------------------------
# General Tests
test-all:
    -cargo test
test-routes:
    -cargo test routes

# Auth Tests
test-auth-action:
    -cargo test auth_action
test-auth-route:
    -cargo test auth_route
test-auth-all:
    just test-auth-route
    just test-auth-action

# Recipe Tests
test-recipe-repo:
    -cargo test recipe_repo
test-recipe-action:
    -cargo test recipe_action
test-recipe-route:
    -cargo test recipe_route
test-recipe-all:
    just test-recipe-repo
    just test-recipe-action
    just test-recipe-route




# Scripts
hit-recipe-api:
    bun ./scripts/spoontacular_api.js
