#!/bin/bash

##
## Configurations
##

# Common shared databases between databases
DATA_ROOT_DIR="db"

# Directory specific databases
DATABASE_ROOT_DIR=".treeleaves"

# Migrations directory
MIGRATIONS_ROOT_DIR="migrations"
DATABASE_TYPE="sqlite3"
MIGRATIONS_DIR="$MIGRATIONS_ROOT_DIR/$DATABASE_TYPE"

# Diesel schema configuration directory
DIESEL_SCHEMAS_DIR="diesel-configs"

# Database urls
DATABASE_URLS=(
    "$DATABASE_ROOT_DIR/main/mime_types.db"
    "$DATABASE_ROOT_DIR/main/files.db"
    "$DATABASE_ROOT_DIR/main/main.db"
    "$DATABASE_ROOT_DIR/types/images.db"
    "$DATABASE_ROOT_DIR/types/videos.db"
    "$DATABASE_ROOT_DIR/types/audio.db"
)

# Diesel database schema mapping configs
DATABASE_SCHEMA_CFGS=()

# Database migrations
MIGRATIONS_DIRS=()


##
## Setup
##

# Setup our database schema configs
for url in ${DATABASE_URLS[@]}; do
    base=$(basename "$url" ".db")
    config="$DIESEL_SCHEMAS_DIR/${base}.toml"
    DATABASE_SCHEMA_CFGS+=("$config")
done

# Setup our migrations directory structure
for url in ${DATABASE_URLS[@]}; do
    base=$(basename "$url" ".db")
    path=$(dirname "$url")
    migration_root_dir="${path/$DATABASE_ROOT_DIR/$MIGRATIONS_DIR}"
    migration_dir="$migration_root_dir/$base"
    MIGRATIONS_DIRS+=("$migration_dir")
done

# Make the migrations hierarchy
for migration_dir in ${MIGRATIONS_DIRS[@]}; do
    if [[ ! -d "$migration_dir" ]]; then
        mkdir -p "$migration_dir"
    fi
done
