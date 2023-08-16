#!/bin/bash

# Setup developer database environment

# Requiremments:
# - `postgres` user has been created

# Setup
DB_CLUSTER_DIR=/var/lib/postgres/data

DB_CLUSTER_SHARED_DIR="/var/lib/postgres/data/dev/treeleaves/shared"
DB_CLUSTER_TARGET_DIR="/var/lib/postgres/data/dev/treeleaves/target"

DB_CLUSTER_SHARED_LOG="$DB_CLUSTER_SHARED_DIR/shared.log"
DB_CLUSTER_TARGET_LOG="$DB_CLUSTER_TARGET_DIR/target.log"

DB_CLUSTER_SHARED_PORT=5450
DB_CLUSTER_TARGET_PORT=5451

PG_USER_ACCOUNT=postgres
PG_DB_USER=treeleaves-dev

# Color Output
#COLOR_GREEN='\e[0;32m'
COLOR_RED='\e[0;31m'
COLOR_NONE='\e[0m'

#sudo -iu "$PG_USER_ACCOUNT"
#sudo -u "$PG_USER_ACCOUNT"

#sudo -iu "$PG_USER_ACCOUNT"
#su -u "$PG_USER_ACCOUNT" -c ./pg_setup_databases.sh
#su -c ./pg_setup_databases.sh -l "$PG_USER_ACCOUNT"

function log() {
    text=$1
    echo -e "${COLOR_RED}[DEBUG]$COLOR_NONE $text"
}


# Check if running as the postgres account user
if [ "$USER" != "$PG_USER_ACCOUNT" ]; then
    # Ensure our
    sudo mkdir -p "$DB_CLUSTER_DIR"
    sudo chown $PG_USER_ACCOUNT:$PG_USER_ACCOUNT -R $DB_CLUSTER_DIR
    # Make the necessary directories first
    #sudo mkdir -p "$DB_CLUSTER_SHARED_DIR"
    #sudo mkdir -p "$DB_CLUSTER_TARGET_DIR"

    ## Ensure our user can write to them
    #sudo chown $PG_USER_ACCOUNT:$PG_USER_ACCOUNT -R $DB_CLUSTER_SHARED_DIR
    #sudo chown $PG_USER_ACCOUNT:$PG_USER_ACCOUNT -R $DB_CLUSTER_TARGET_DIR


    # Login to postgres user account
    log "Logging in to postgres user account: ${PG_USER_ACCOUNT}"
    su -s /bin/bash -c ./pg_setup_databases.sh "$PG_USER_ACCOUNT"

    #log "Please run as the postgres user account: \"${PG_USER_ACCOUNT}\""
    exit
fi
log "Now running as: $PG_USER_ACCOUNT"

# Suppress errors "could not change directory to $cwd: Permission denied"
cd $HOME

# Create clusters
log "Creating database clusters"

initdb -D "$DB_CLUSTER_SHARED_DIR"
initdb -D "$DB_CLUSTER_TARGET_DIR"

# Start our clusters
log "Starting database clusters"
pg_ctl -D "$DB_CLUSTER_SHARED_DIR" -l "$DB_CLUSTER_SHARED_LOG" -o "-p $DB_CLUSTER_SHARED_PORT" start
pg_ctl -D "$DB_CLUSTER_TARGET_DIR" -l "$DB_CLUSTER_TARGET_LOG" -o "-p $DB_CLUSTER_TARGET_PORT" start

# Create our postgres database user
log "Creating database user: $PG_DB_USER"

#createuser "$PG_DB_USER"
createuser "$PG_DB_USER" -p $DB_CLUSTER_TARGET_PORT
createuser "$PG_DB_USER" -p $DB_CLUSTER_SHARED_PORT

# Create all our development databases
log "Creating databases"

# We will reverse these ports for treeleaves
# Development:
#   5500+ (Shared)
#   5550+ (Target)
# Production:
#   5600+ (Shared)
#   5650+ (Target)

DBS_SHARED=(
    "$DB_CLUSTER_SHARED_DIR/data/mime_types.db"
)

DBS_TARGET=(
    "$DB_CLUSTER_TARGET_DIR/main/files.db"
    "$DB_CLUSTER_TARGET_DIR/main/main.db"
    "$DB_CLUSTER_TARGET_DIR/types/images.db"
    "$DB_CLUSTER_TARGET_DIR/types/videos.db"
    "$DB_CLUSTER_TARGET_DIR/types/audio.db"
)

PORT_SHARED=5500
PORT_TARGET=5550

HOST=localhost

function create_all_dbs() {
    host=$1
    user=$2
    port=$3
    shift; shift; shift;
    dbs=("$@")

    for path in ${dbs[@]}; do
        dir=$(dirname "$path")
        dbname=$(basename "$path" ".db")
        if [[ -z "$dbname" ]]; then
            break;
        fi
        # Create the database in the given tablespaces directory
        #log """
        #createdb \\
            #-h "$host" \\
            #-U "$user" \\
            #-p "$port" \\
            #"$dbname"
        #"""
        log """
        createdb \\
            -h "$host" \\
            -U "$user" \\
            -p "$port" \\
            "$dbname"
        """


        createdb \
            -h "$host" \
            -U "$user" \
            -p "$port" \
            "$dbname"

        #let "port++"
    done
}

#create_all_dbs "$HOST" "$PG_USER_ACCOUNT" "$PORT_SHARED" "${DBS_SHARED[@]}"
#create_all_dbs "$HOST" "$PG_USER_ACCOUNT" "$PORT_TARGET" "${DBS_TARGET[@]}"

create_all_dbs "$HOST" "$PG_USER_ACCOUNT" "$DB_CLUSTER_SHARED_PORT" "${DBS_SHARED[@]}"
create_all_dbs "$HOST" "$PG_USER_ACCOUNT" "$DB_CLUSTER_TARGET_PORT" "${DBS_TARGET[@]}"


# Stop the database clusters when finished
pg_ctl stop -D "$DB_CLUSTER_SHARED_DIR"
pg_ctl stop -D "$DB_CLUSTER_TARGET_DIR"
