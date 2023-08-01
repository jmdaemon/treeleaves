#!/bin/bash

# Setup the initial databases migrations

# NOTE:
# Diesel must use the same directory structure as our databases structure/
# or else diesel will run the same migrations for all databases tables


# Setup the migrations directory if it doesn't exist
#if [[ ! -d "$MIGRATIONS_DIR" ]]; then 
    #mkdir -p "$MIGRATIONS_DIR"
#fi

source config.sh

# Populate migrations
MIGRATIONS_DIRS=()
MIGRATIONS=()

# Get the parent directory of a path
function parent() {
    echo "$(builtin cd $1; pwd)"
}

# Populate migrations directories
for url in ${DATABASE_URLS[@]}; do
    # Replace ".treeleaves/main/files.db" -> "migrations/sqlite3/main/files"

    #dir=$(parent "$url")

    base=$(basename "$url" ".db")

    #dir=$(dirname "$url")
    #migration_dir="${dir/\$DATABASE_ROOT_DIR/$MIGRATIONS_DIR}"

    path=$(dirname "$url")
    migration_root_dir="${path/$DATABASE_ROOT_DIR/$MIGRATIONS_DIR}"
    migration_dir="$migration_root_dir/$base"

    echo "$migration_dir"


    
    MIGRATIONS_DIRS+=("$migration_dir")
done

# Make necessary migrations directory structure
for migration_dir in ${MIGRATIONS_DIRS[@]}; do
    # Make directory if not already exists
    if [[ ! -d "$migration_dir" ]]; then
        mkdir -p "$migration_dir"
    fi
done



# Populate migrations
for url in ${DATABASE_URLS[@]}; do
    base=$(basename "$url" ".db")
    migration="create_${base}"
    #${firstString/Suzi/"$secondString"}"

    # Setup directory structure
    MIGRATIONS+=("$migration")
done

# Create the migrations
#for migration in ${MIGRATIONS[@]}; do
for i in "${!MIGRATIONS[@]}"; do
    migration_dir="${MIGRATIONS_DIRS[i]}"
    migration="${MIGRATIONS[i]}"

    echo """Running:
    diesel  migration \\
            --migration-dir "$migration_dir" \\
            generate "$migration"
    """

    diesel  migration \
            --migration-dir "$migration_dir" \
            generate  "$migration"
done

#for migration in ${MIGRATIONS[@]}; do
    #diesel  migration \
            #--migration-dir "$MIGRATIONS_DIR" \
            #generate  "$migration"
#done
