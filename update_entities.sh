#!/bin/bash
set -a
source .env
set +a

sea-orm-cli generate entity -u "$DATABASE_URL" -o src/entity