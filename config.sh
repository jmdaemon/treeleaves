#!/bin/bash

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
