#!/bin/bash

source config.sh
run_as "$PG_USER_ACCOUNT" "$0"
suppress_cd_error

log "Starting database clusters"
pg_ctl -D "$DB_CLUSTER_SHARED_DIR" -l "$DB_CLUSTER_SHARED_LOG" -o "-p $DB_CLUSTER_SHARED_PORT" stop
pg_ctl -D "$DB_CLUSTER_TARGET_DIR" -l "$DB_CLUSTER_TARGET_LOG" -o "-p $DB_CLUSTER_TARGET_PORT" stop
