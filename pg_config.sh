#!/bin/bash

# Postgres Development Configuration

## Postgres Database Cluster
DB_CLUSTER_DIR=/var/lib/postgres/data

DB_CLUSTER_SHARED_DIR="/var/lib/postgres/data/dev/treeleaves/shared"
DB_CLUSTER_TARGET_DIR="/var/lib/postgres/data/dev/treeleaves/target"

DB_CLUSTER_SHARED_LOG="$DB_CLUSTER_SHARED_DIR/shared.log"
DB_CLUSTER_TARGET_LOG="$DB_CLUSTER_TARGET_DIR/target.log"

DB_CLUSTER_SHARED_PORT=5450
DB_CLUSTER_TARGET_PORT=5451

PG_USER_ACCOUNT=postgres
PG_DB_USER=treeleaves-dev

HOST=localhost

## Postgres Databases
DBS_SHARED=(
    "$DB_CLUSTER_SHARED_DIR/data/mime_types.db"
)

DBS_TARGET=(
    "$DB_CLUSTER_TARGET_DIR/main/files.db"
    "$DB_CLUSTER_TARGET_DIR/main/main.db"
    "$DB_CLUSTER_TARGET_DIR/types/images.db"
    "$DB_CLUSTER_TARGET_DIR/types/videos.db"
    "$DB_CLUSTER_TARGET_DIR/types/audio.db"
)

# Colors
COLOR_RED='\e[0;31m'
COLOR_NONE='\e[0m'

source lib.sh
