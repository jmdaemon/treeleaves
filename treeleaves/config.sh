#!/bin/bash

# Postgres Development Configuration

## Config

## Postgres Databases
DBS_SHARED=(
    "data/mime_types"
)

DBS_TARGET=(
    "main/files"
    "types/images"
    "types/videos"
    "types/audio"
)

DATABASE_TYPE="postgres"

## Diesel Migrations

MIGRATIONS_ROOT_DIR="migrations"
MIGRATIONS_DIR="$MIGRATIONS_ROOT_DIR/$DATABASE_TYPE"

## Diesel Schemas

DIESEL_SCHEMAS_ROOT_DIR="diesel-configs"
DIESEL_SCHEMAS_DIR="$DIESEL_SCHEMAS_ROOT_DIR/$DATABASE_TYPE"

## Postgres Database Cluster

### Directories
DB_CLUSTER_DIR=/var/lib/postgres/data
DB_CLUSTER_LOCK_DIR=/run/postgresql

DB_CLUSTER_SHARED_DIR="/var/lib/postgres/data/dev/treeleaves/shared"
DB_CLUSTER_TARGET_DIR="/var/lib/postgres/data/dev/treeleaves/target"

DB_CLUSTER_SHARED_LOG="$DB_CLUSTER_SHARED_DIR/shared.log"
DB_CLUSTER_TARGET_LOG="$DB_CLUSTER_TARGET_DIR/target.log"

# We will reverse these ports for treeleaves
# Development:
#   5500+ (Shared)
#   5550+ (Target)
# Production:
#   5600+ (Shared)
#   5650+ (Target)

DB_CLUSTER_SHARED_PORT=5450
DB_CLUSTER_TARGET_PORT=5451

PG_USER_ACCOUNT=postgres
PG_DB_USER=treeleaves-dev

HOST=localhost

### URLs

DB_CLUSTER_SHARED_URL="postgresql://$PG_USER_ACCOUNT@$HOST:$DB_CLUSTER_SHARED_PORT"
DB_CLUSTER_TARGET_URL="postgresql://$PG_USER_ACCOUNT@$HOST:$DB_CLUSTER_TARGET_PORT"

# Colors
COLOR_RED='\e[0;31m'
COLOR_NONE='\e[0m'

source lib.sh
