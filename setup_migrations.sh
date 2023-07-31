#!/bin/bash

# Setup the initial database migrations

# Make the migrations directories
MIGRATIONS_ROOT_DIR="migrations"
DATABASE_TYPE="sqlite3"

migrations=(
    "create_mime_types"
    "create_files"
    "create_main"
    "create_images"
    "create_audio"
    "create_videos"
    )

# Create the migrations root directory
MIGRATIONS_DIR="$MIGRATIONS_ROOT_DIR/$DATABASE_TYPE"
if [[ ! -d "$MIGRATIONS_DIR" ]]; then 
    mkdir -p "$MIGRATIONS_DIR"
fi

for migration in ${migrations[@]}; do
    diesel migration generate --migration-dir "$MIGRATIONS_DIR" "$migration"
done
