#!/bin/bash
set -e

# configure user
psql -v ON_ERROR_STOP=1 --username postgres <<-EOSQL
	CREATE USER splinter WITH PASSWORD 'cheese';
	CREATE DATABASE turtles OWNER splinter;
	GRANT ALL PRIVILEGES ON DATABASE turtles TO splinter;

    \connect turtles
    GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO splinter;
EOSQL

