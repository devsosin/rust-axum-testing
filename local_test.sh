#!/bin/bash
set -e

TARGET=$1

export DATABASE_URL=postgres://test:test1234@localhost:5432/test_db

# echo $DATABASE_URL

cargo test domain::$TARGET 
