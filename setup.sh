#!/bin/bash

# Make the necessary database directory structure
DATABASE_ROOT_DIR=".treeleaves"
#MIGRATIONS_ROOT_DIR="migrations"
hierarchy=(main types features addons)

for dir in ${hierarchy[@]}; do
    mkdir -p "$DATABASE_ROOT_DIR/$dir"
    #mkdir -p "$MIGRATIONS_ROOT_DIR/$dir"
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

# Make the migrations directories
MIGRATIONS_ROOT_DIR="migrations"
DATABASE_TYPE="sqlite3"

migrations=(
    "create_files"
    "create_main"
    "create_images"
    "create_audio"
    "create_videos"
    )

MIGRATIONS_DIR="$MIGRATIONS_ROOT_DIR/$DATABASE_TYPE"
mkdir -p "$MIGRATIONS_DIR"

for migration in ${migrations[@]}; do
    #dir="$MIGRATIONS_ROOT_DIR/$DATABASE_TYPE/$migration"
    diesel migration generate --migration-dir "$MIGRATIONS_DIR" "$migration"
    #mkdir -p "$dir"
done


# Create initial migrations database files
# diesel migration run --config-file=diesel1.toml
# diesel migration run --config-file=diesel2.toml
