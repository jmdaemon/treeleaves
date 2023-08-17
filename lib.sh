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
