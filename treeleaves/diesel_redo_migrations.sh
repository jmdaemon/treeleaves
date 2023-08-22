#!/bin/bash

# Setup database migrations

source config.sh

DB_MIGRATIONS_SHARED=()
DB_MIGRATIONS_TARGET=()

pop_migrations DB_MIGRATIONS_SHARED "${DBS_SHARED[@]}"
pop_migrations DB_MIGRATIONS_TARGET "${DBS_TARGET[@]}"

run_migrations "redo" "$DB_CLUSTER_SHARED_URL" "${DB_MIGRATIONS_SHARED[@]}"
run_migrations "redo" "$DB_CLUSTER_TARGET_URL" "${DB_MIGRATIONS_TARGET[@]}"
