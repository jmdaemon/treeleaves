#!/bin/bash

# Reruns the databases migrations

source config.sh

# Populate migrations
MIGRATIONS_DIRS=()

# Populate migrations directories
for url in ${DATABASE_URLS[@]}; do
    base=$(basename "$url" ".db")

    path=$(dirname "$url")
    migration_root_dir="${path/$DATABASE_ROOT_DIR/$MIGRATIONS_DIR}"
    migration_dir="$migration_root_dir/$base"

    MIGRATIONS_DIRS+=("$migration_dir")
done

# Create the migrations
for i in "${!MIGRATIONS_DIRS[@]}"; do
    url="${DATABASE_URLS[i]}"
    migration_dir="${MIGRATIONS_DIRS[i]}"

    echo """Running:
    diesel  --database-url "$url" \\
            migration \\
            --migration-dir "$migration_dir" \\
            redo
    """

    diesel  --database-url "$url" \
            migration \
            --migration-dir "$migration_dir" \
            redo
done
