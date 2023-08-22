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
        # Ensure /var/lib/postgres/data is owned by our user
        sudo mkdir -p "$DB_CLUSTER_DIR"
        sudo chown $PG_USER_ACCOUNT:$PG_USER_ACCOUNT -R $DB_CLUSTER_DIR

        # Ensure /run/postgresql is owned by our user
        sudo mkdir -p "$DB_CLUSTER_LOCK_DIR"
        sudo chown $PG_USER_ACCOUNT:$PG_USER_ACCOUNT -R "$DB_CLUSTER_LOCK_DIR"

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

# Migrations
function pop_migrations() {
    declare -n migrations=$1
    shift;
    db_urls=("$@")
    for url in ${db_urls[@]}; do
        base=$(basename "$url")
        path=$(dirname "$url")
        migration_dir="$MIGRATIONS_DIR/$path/$base"
        migrations+=("$migration_dir")
    done
}

# Setup all migrations
function run_migrations() {
    choice="$1"
    url=$2
    shift; shift;
    migrations=("$@")

    run_as=""
    if [[ "$choice" == "run" ]]; then
        run_as=run
    elif [[ "$choice" == "redo" ]]; then
        run_as=redo
    else
        log "Usage: run_migrations [run|redo] [db_cluster_url] [db_migrations_array]"
        exit 1;
    fi

    for path in ${migrations[@]}; do
        base=$(basename "$path")
        name="create_${base}"


    log """Running:
        diesel  --database-url "$url" \\
                migration \\
                --migration-dir "$path" \\
                $run_as
        """

    diesel  --database-url "$url" \
            migration \
            --migration-dir "$path" \
            $run_as
    done
    log "Ran all migrations"
}

function pop_schemas() {
    declare -n schemas=$1
    shift;
    tables=("$@")
    for table in "${tables[@]}"; do
        config="$DIESEL_SCHEMAS_DIR/${table}.toml"
        schemas+=("$config")
    done
}

# Runs the migration and generates the schema file
function setup_schemas() {
    url=$1
    declare -n migrations=$2
    shift; shift;
    configs=("$@")

    for i in ${!configs[@]}; do
        cfg="${configs[i]}"
        migration="${migrations[i]}"
        log """Running:
        diesel  --database-url "$url" \\
                --config-file "$cfg" \\
                migration \\
                --migration-dir "$migration" \\
                run
        """

        diesel  --database-url "$url" \
                --config-file "$cfg" \
                migration \
                --migration-dir "$migration_dir" \
                run
    done
    log "Generated all database schemas"
}
