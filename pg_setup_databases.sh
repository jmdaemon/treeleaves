#!/bin/bash

# Setup developer database environment

# Requiremments:
# - `postgres` user has been created

# Setup
DB_CLUSTER_TARGET_DIR="/var/lib/postgres/data/dev/treeleaves/target"
DB_CLUSTER_SHARED_DIR="/var/lib/postgres/data/dev/treeleaves/shared"
PG_USER_ACCOUNT=postgres
PG_DB_USER=treeleaves-dev

# Login to postgres user account
#sudo -iu "$PG_USER_ACCOUNT"

# Check if running as the postgres account user
#if [ "$USER" != "$PG_USER_ACCOUNT" ]; then
    #echo "Please run as the postgres user account: \"${PG_USER_ACCOUNT}\""
    #exit
#fi
# Now running as $PG_USER_ACCOUNT

# Create clusters
#initdb -D "$DB_CLUSTER_SHARED_DIR"
#initdb -D "$DB_CLUSTER_TARGET_DIR"

# Create our postgres database user
#createuser "$PG_DB_USER"

# Create all our development databases

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
    #dbs=$1
    #dbs=("$@")
    #host=$2
    #port=$3
    #user=$4

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
        echo """
        createdb \\
            -h "$host" \\
            -U "$user" \\
            -D "$dir" \\
            -p "$port" \\
            "$dbname"
        """

        #createdb \
            #-D "$dir" \
            #-p "$port" \
            #-U "$user" \
            #"$dbname"

        let "port++"
    done
}

#create_all_dbs "${DBS_SHARED[@]}" "$HOST" "$PORT_SHARED" "$PG_USER_ACCOUNT"
#create_all_dbs "${DBS_TARGET[@]}" "$HOST" "$PORT_TARGET" "$PG_USER_ACCOUNT"

create_all_dbs "$HOST" "$PG_USER_ACCOUNT" "$PORT_SHARED" "${DBS_SHARED[@]}"
create_all_dbs "$HOST" "$PG_USER_ACCOUNT" "$PORT_TARGET" "${DBS_TARGET[@]}"
