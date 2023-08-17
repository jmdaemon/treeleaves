#!/bin/bash

# Log with color
function log() {
    text=$1
    echo -e "${COLOR_RED}[DEBUG]$COLOR_NONE $text"
}

function run_as() {
    user=$1
    script=$2

    # Check if running as the postgres account user
    if [ "$USER" != "$PG_USER_ACCOUNT" ]; then
        # Ensure our parent directory is owned by our user
        sudo mkdir -p "$DB_CLUSTER_DIR"
        sudo chown $PG_USER_ACCOUNT:$PG_USER_ACCOUNT -R $DB_CLUSTER_DIR

        # Login to postgres user account
        log "Logging in to postgres user account: ${PG_USER_ACCOUNT}"

        # Load the current file script
        #su -s /bin/bash -c $0 "$PG_USER_ACCOUNT"
        su -s /bin/bash -c $script "$PG_USER_ACCOUNT"

        exit # Exit our spawned shell later
    fi
    log "Now running as: $PG_USER_ACCOUNT"
}

function suppress_cd_error() {
    # Suppress errors "could not change directory to $cwd: Permission denied"
    cd $HOME
}

function create_all_dbs() {
    host=$1
    user=$2
    port=$3
    db_cluster_dir=$4
    shift; shift; shift; shift;
    dbs=("$@")

    for path in ${dbs[@]}; do
        dir=$(dirname "$path")
        dbpath=$(basename "$path")
        dbname="$db_cluster_dir/$dbpath"
        if [[ -z "$dbname" ]]; then
            break;
        fi
        # Create the database in the given tablespaces directory
        log """
        createdb \\
            -h "$host" \\
            -U "$user" \\
            -p "$port" \\
            "$dbname"
        """

        #createdb \
            #-h "$host" \
            #-U "$user" \
            #-p "$port" \
            #"$dbname"
    done
}

function pop_migrations() {
    declare -n migrations=$1
    shift;
    db_urls=("$@")
    for url in ${db_urls[@]}; do
        base=$(basename "$url" ".db")
        path=$(dirname "$url")
        #migration_root_dir="${path/$DATABASE_ROOT_DIR/$MIGRATIONS_DIR}"
        migration_root_dir="${path/$DATABASE_ROOT_DIR/$MIGRATIONS_DIR}"
        migration_dir="$migration_root_dir/$base"
        migrations+=("$migration_dir")
    done
}
