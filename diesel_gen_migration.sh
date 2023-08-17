#!/bin/bash

source pg_config.sh

# Handle cli arguments
database_cluster="$1"
migration_name=$2
shift; shift;

# Shared or Target database cluster
url=""
if [[ "$database_cluster" == "shared" ]]; then
    url="$DB_CLUSTER_SHARED_URL"
elif [[ "$database_cluster" == "target" ]]; then
    url="$DB_CLUSTER_TARGET_URL"
else
    log "Usage: $0 \"[shared|target] migration_name\""
    exit 1
fi

migration_dir="$MIGRATIONS_DIR/$migration_name"

log """Running
diesel --database-url "$url" \\
       setup \\
       --migration-dir "$migration_dir"
"""

diesel --database-url "$url" \
       setup \
       --migration-dir "$migration_dir"
