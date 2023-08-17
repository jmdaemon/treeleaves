#!/bin/bash

source config.sh
source pg_config.sh

#DB_MIGRATIONS_SHARED=(
#)

#migration_dir="$migration_root_dir/$base"

DB_CLUSTER_SHARED_URL="postgresql://$PG_USER_ACCOUNT@$HOST:$DB_CLUSTER_SHARED_PORT"
DB_CLUSTER_TARGET_URL="postgresql://$PG_USER_ACCOUNT@$HOST:$DB_CLUSTER_TARGET_PORT"

# Setup all migrations
function setup_migrations() {
    url=$1
    shift;
    migrations=("$@")
    for path in ${migrations[@]}; do
        log """Running
        diesel --database-url "$url" \\
               setup \\
               --migration-dir "$path"
        """

        diesel --database-url "$url" \
               setup \
               --migration-dir "$path"
    done
    log "Setup all migrations"
}

#setup_migrations "$DB_CLUSTER_SHARED_URL" "${DBS_SHARED[@]}"
#setup_migrations "$DB_CLUSTER_TARGET_URL" "${DBS_TARGET[@]}"

#setup_migrations "$DB_CLUSTER_SHARED_URL" "${DB_URLS_SHARED[@]}"
#setup_migrations "$DB_CLUSTER_TARGET_URL" "${DB_URLS_TARGET[@]}"

setup_migrations "$DB_CLUSTER_SHARED_URL" "${DB_MIGRATIONS_SHARED[@]}"
setup_migrations "$DB_CLUSTER_TARGET_URL" "${DB_MIGRATIONS_TARGET[@]}"
