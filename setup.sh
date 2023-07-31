#!/bin/bash

# Make the necessary database directory structure
DATABASE_ROOT_DIR=".treeleaves"
hierarchy=(main types features addons)

for dir in ${hierarchy[@]}; do
    mkdir -p "$DATABASE_ROOT_DIR/$dir"
done

# Setup the databases with diesel-cli
ENV_DIR=".env"

databases=(
    "files"
    "main"
    "images"
    "videos"
    "audio"
)

for database in ${databases[@]}; do
    dir="$ENV_DIR/$database"
    if [[ -f "$dir" ]]; then
        diesel --database-url $(cat "$dir") setup
    fi
done
