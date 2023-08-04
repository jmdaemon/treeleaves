#!/bin/bash

# Generate the rust schema mappings for our databases

# Load configuration variables
source config.sh

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
