#!/bin/bash

./pg_setup_databases.sh && ./pg_start_databases.sh && ./diesel_setup_migrations.sh
