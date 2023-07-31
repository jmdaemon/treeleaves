#!/bin/bash

# Setup the necessary database hierarchy

# Make the necessary database directory structure
DATABASE_ROOT_DIR=".treeleaves"
hierarchy=(main types features addons)

for dir in ${hierarchy[@]}; do
    mkdir -p "$DATABASE_ROOT_DIR/$dir"
done

# Setup the databases with diesel-cli
ENV_DIR=".env"

databases=(
    "mime_types"
    "files"
    "main"
    "images"
    "videos"
    "audio"
)

for database in ${databases[@]}; do
    db_url_file="$ENV_DIR/$database"
    if [[ -f "$db_url_file" ]]; then
        db=$(cat "$db_url_file")
        if [[ ! -f "$db" ]]; then
            diesel --database-url "$db" setup
        fi
    fi
done
