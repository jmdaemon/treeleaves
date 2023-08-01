#!/bin/bash

# Setup the necessary databases hierarchy

source config.sh

# Make the necessary database directory structure
for url in ${DATABASE_URLS[@]}; do
    dir=$(dirname "$url")
    mkdir -p "$dir"
done

# Setup the databases with diesel-cli
for i in "${!DATABASE_URLS[@]}"; do
    migration_dir="${MIGRATIONS_DIRS[i]}"
    url="${DATABASE_URLS[i]}"
    if [[ ! -f "$url" ]]; then
        echo """Running
        diesel --database-url "$url" \\
               setup \\
               --migration-dir "$migration_dir"

        """
        diesel --database-url "$url" \
               setup \
               --migration-dir "$migration_dir"
    fi
done
