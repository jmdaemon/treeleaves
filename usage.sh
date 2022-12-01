#!/bin/bash

DB_NAME="test.db"
DB_TYPE="imagedb"
IMAGE_ID="6570239"
PBOORU="ponerpics"
CWD="."

# Standalone Binary
treeleaves create $DB_NAME $DB_TYPE
treeleaves show $DB_NAME
treeleaves populate $DB_NAME $CWD
treeleaves fetch $DB_NAME $IMAGE_ID $PBOORU

# Cargo
# cargo run -- create $DB_NAME
# cargo run -- show $DB_NAME
# cargo run -- populate $DB_NAME $CWD
# cargo run -- $DB_NAME $IMAGE_ID $PBOORU
