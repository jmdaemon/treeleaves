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

DATABASE_URLS=(
    "$DATABASE_ROOT_DIR/main/mime_types.db"
    "$DATABASE_ROOT_DIR/main/files.db"
    "$DATABASE_ROOT_DIR/main/main.db"
    "$DATABASE_ROOT_DIR/types/images.db"
    "$DATABASE_ROOT_DIR/types/videos.db"
    "$DATABASE_ROOT_DIR/types/audio.db"
)
#DATABASE_MIGRATIONS=()
DATABASE_SCHEMA_CFGS=()

# Populate the database migrations
#for url in ${DATABASE_URLS[@]}; do
    #migration=$(basename "")
    #DATABASE_MIGRATIONS+=
#done

# Populate the list of database schema configs
for url in ${DATABASE_URLS[@]}; do
    #base=$(basename "$url")
    #base="${url%.*}"
    base=$(basename "$url" ".db")
    config="$DIESEL_SCHEMAS_DIR/${base}.toml"
    DATABASE_SCHEMA_CFGS+=("$config")
done

#DATABASE_URLS


#diesel  --database-url .treeleaves/main/mime_types.db \
        #--config-file diesel-configs/mime_types.toml \
        #migration --migration-dir migrations/sqlite3 \
        #run
#for 

# Generate the schema file from the migration
for i in "${!DATABASE_URLS[@]}"; do
    url="${DATABASE_URLS[i]}"
    cfg="${DATABASE_SCHEMA_CFGS[i]}"

    echo """Running:
    diesel  --database-url "$url" \\
            --config-file "$cfg" \\
            migration \\
            --migration-dir "$MIGRATIONS_DIR" \\
            run
    """


    diesel  --database-url "$url" \
            --config-file "$cfg" \
            migration \
            --migration-dir "$MIGRATIONS_DIR" \
            run
            #run
            #generate
done
