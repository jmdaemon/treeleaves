#!/bin/bash

diesel  --database-url .treeleaves/main/mime_types.db \
        --config-file diesel-configs/mime_types.toml \
        migration --migration-dir migrations/sqlite3 \
        run
