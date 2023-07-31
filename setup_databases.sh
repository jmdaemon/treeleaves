#!/bin/bash

# Setup the necessary databases hierarchy

source config.sh

# Make the necessary database directory structure
for url in ${DATABASE_URLS[@]}; do
    dir=$(dirname "$url")
    mkdir -p "$dir"
done

# Setup the databases with diesel-cli
for url in ${DATABASE_URLS[@]}; do
    if [[ ! -f "$url" ]]; then
        diesel --database-url "$url" setup
    fi
done
