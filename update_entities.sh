#!/bin/bash
set -a
source .env
set +a

OUTPUT_DIR="src/entity"

# No argument
if [ -z "$1" ]; then
  echo "Usage:"
  echo "  $0 <table_name>"
  echo "  $0 clean"
  exit 1
fi

# CLEAN MODE
if [ "$1" == "clean" ]; then
  echo "WARNING: This will delete all generated entities in $OUTPUT_DIR"
  read -p "Are you sure you want to clean? (y/N): " confirm

  if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
    echo "Cancelled."
    exit 0
  fi

  echo "Cleaning $OUTPUT_DIR..."
  rm -rf "$OUTPUT_DIR"/*
  echo "Done."
  exit 0
fi

# GENERATE SINGLE TABLE
TABLE=$1

sea-orm-cli generate entity \
  -u "$DATABASE_URL" \
  -o "$OUTPUT_DIR" \
  --tables "$TABLE"