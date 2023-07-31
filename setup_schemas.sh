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

# Generate the schema file from the migration
for i in "${!DATABASE_URLS[@]}"; do
    url="${DATABASE_URLS[i]}"
    cfg="${DATABASE_SCHEMA_CFGS[i]}"

    echo """Running:
    diesel  --database-url "$url" \\
            --config-file "$cfg" \\
            migration \\
            --migration-dir "$MIGRATIONS_DIR" \\
            run
    """

    diesel  --database-url "$url" \
            --config-file "$cfg" \
            migration \
            --migration-dir "$MIGRATIONS_DIR" \
            run
done
