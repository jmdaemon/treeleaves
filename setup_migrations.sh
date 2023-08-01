#!/bin/bash

# Setup the initial databases migrations

# NOTE:
# Diesel must use the same directory structure as our databases structure/
# or else diesel will run the same migrations for all databases tables

source config.sh

# Populate migrations
MIGRATIONS=()

# Populate migrations
for url in ${DATABASE_URLS[@]}; do
    base=$(basename "$url" ".db")
    migration="create_${base}"
    MIGRATIONS+=("$migration")
done

# Create the migrations
for i in "${!MIGRATIONS[@]}"; do
    migration_dir="${MIGRATIONS_DIRS[i]}"
    migration="${MIGRATIONS[i]}"

    echo """Running:
    diesel  migration \\
            --migration-dir "$migration_dir" \\
            generate "$migration"
    """

    diesel  migration \
            --migration-dir "$migration_dir" \
            generate  "$migration"
done
