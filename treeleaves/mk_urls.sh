#!/bin/bash

source config.sh

dest=src
echo "$DB_CLUSTER_SHARED_URL" > "$dest/url.shared"
echo "$DB_CLUSTER_TARGET_URL" > "$dest/url.target"
