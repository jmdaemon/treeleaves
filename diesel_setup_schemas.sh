#!/bin/bash

# Setup database migrations

source config.sh

# Populate migrations
DB_MIGRATIONS_SHARED=()
DB_MIGRATIONS_TARGET=()

pop_migrations DB_MIGRATIONS_SHARED "${DBS_SHARED[@]}"
pop_migrations DB_MIGRATIONS_TARGET "${DBS_TARGET[@]}"

# Populate schema configs

DB_SCHEMA_CONFIGS_SHARED=()
DB_SCHEMA_CONFIGS_TARGET=()

pop_schemas DB_SCHEMA_CONFIGS_SHARED "${DBS_SHARED[@]}"
pop_schemas DB_SCHEMA_CONFIGS_TARGET "${DBS_TARGET[@]}"

# Run the migrations to generate the schemas
setup_schemas "$DB_CLUSTER_SHARED_URL" DB_MIGRATIONS_SHARED "${DB_SCHEMA_CONFIGS_SHARED[@]}"
setup_schemas "$DB_CLUSTER_TARGET_URL" DB_MIGRATIONS_TARGET "${DB_SCHEMA_CONFIGS_TARGET[@]}"
