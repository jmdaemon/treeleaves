#!/bin/bash

# Setup the initial databases migrations

# Setup the migrations directory if it doesn't exist
if [[ ! -d "$MIGRATIONS_DIR" ]]; then 
    mkdir -p "$MIGRATIONS_DIR"
fi

# Populate migrations
MIGRATIONS=()

for url in ${DATABASE_URLS[@]}; do
    base=$(basename "$url" ".db")
    migration="create_${base}"
    MIGRATIONS+=("$migration")
done

# Create the migrations
for migration in ${MIGRATIONS[@]}; do
    diesel  migration \
            --migration-dir "$MIGRATIONS_DIR" \
            generate  "$migration"
done
