#!/bin/bash

# Setup developer database environment

# Requiremments:
# - `postgres` user has been created

# Setup
source pg_config.sh
run_as "$PG_USER_ACCOUNT" "$0"
suppress_cd_error

log "Creating database clusters"

initdb -D "$DB_CLUSTER_SHARED_DIR"
initdb -D "$DB_CLUSTER_TARGET_DIR"

log "Starting database clusters"
pg_ctl -D "$DB_CLUSTER_SHARED_DIR" -l "$DB_CLUSTER_SHARED_LOG" -o "-p $DB_CLUSTER_SHARED_PORT" start
pg_ctl -D "$DB_CLUSTER_TARGET_DIR" -l "$DB_CLUSTER_TARGET_LOG" -o "-p $DB_CLUSTER_TARGET_PORT" start

log "Creating postgres database user: $PG_DB_USER"
createuser "$PG_DB_USER" -p $DB_CLUSTER_TARGET_PORT
createuser "$PG_DB_USER" -p $DB_CLUSTER_SHARED_PORT

# Create all our development databases
log "Creating databases"

create_all_dbs "$HOST" "$PG_USER_ACCOUNT" "$DB_CLUSTER_SHARED_PORT" "$DB_CLUSTER_SHARED_DIR" "${DBS_SHARED[@]}"
create_all_dbs "$HOST" "$PG_USER_ACCOUNT" "$DB_CLUSTER_TARGET_PORT" "$DB_CLUSTER_TARGET_DIR" "${DBS_TARGET[@]}"

log "Shutting down clusters"
pg_ctl stop -D "$DB_CLUSTER_SHARED_DIR"
pg_ctl stop -D "$DB_CLUSTER_TARGET_DIR"
