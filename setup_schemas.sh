#!/bin/bash

# Generate the rust schema mappings for our databases

# Load configuration variables
source config.sh

# Populate the list of database schema configs
for url in ${DATABASE_URLS[@]}; do
    base=$(basename "$url" ".db")
    config="$DIESEL_SCHEMAS_DIR/${base}.toml"
    DATABASE_SCHEMA_CFGS+=("$config")
done

MIGRATIONS_DIRS=()

# Populate migrations directories
for url in ${DATABASE_URLS[@]}; do
    base=$(basename "$url" ".db")

    path=$(dirname "$url")
    migration_root_dir="${path/$DATABASE_ROOT_DIR/$MIGRATIONS_DIR}"
    migration_dir="$migration_root_dir/$base"

    MIGRATIONS_DIRS+=("$migration_dir")
done

# Generate the schema file from the migration
for i in "${!DATABASE_URLS[@]}"; do
    url="${DATABASE_URLS[i]}"
    migration_dir="${MIGRATIONS_DIRS[i]}"
    cfg="${DATABASE_SCHEMA_CFGS[i]}"

    echo """Running:
    diesel  --database-url "$url" \\
            --config-file "$cfg" \\
            migration \\
            --migration-dir "$migration_dir" \\
            run
    """

    diesel  --database-url "$url" \
            --config-file "$cfg" \
            migration \
            --migration-dir "$migration_dir" \
            run
done
