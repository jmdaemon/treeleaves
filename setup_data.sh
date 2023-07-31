#!/bin/bash

MIME_JSON_URL="https://cdn.jsdelivr.net/gh/jshttp/mime-db@1.52.0/db.json"
DATA_ROOT_DIR="data"

if [[ ! -d "$DATA_ROOT_DIR" ]]; then
    mkdir -p "$DATA_ROOT_DIR"
fi

wget -P "$DATA_ROOT_DIR" "$MIME_JSON_URL"

# Process the json
