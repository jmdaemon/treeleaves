#!/bin/bash

source pg_config.sh

# Start our database clusters
pg_ctl -D "$DB_CLUSTER_SHARED_DIR" -l "$DB_CLUSTER_SHARED_LOG" -o "-p $DB_CLUSTER_SHARED_PORT" start
pg_ctl -D "$DB_CLUSTER_TARGET_DIR" -l "$DB_CLUSTER_TARGET_LOG" -o "-p $DB_CLUSTER_TARGET_PORT" start
