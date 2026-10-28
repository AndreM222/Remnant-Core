#!/bin/bash

if ! command -v sea-orm-cli &> /dev/null; then
    echo "⚠️ sea-orm-cli is required for this action"
    echo "Setting required tools"
    ./setup.sh

    if [ $? -ne 0 ]; then
        echo "❌ Failed to setup sea-orm-cli."
        exit 1
    fi

    echo "✅ Setup completed."
fi

echo "Generating entities..."
sea-orm-cli generate entity -o src/db/entities/

if [ $? -ne 0 ]; then
    echo "❌ Failed to update entities."
    exit 1
fi

echo "✅ Entities generated."

echo "Migrating to database..."
sea-orm-cli migrate up

if [ $? -ne 0 ]; then
    echo "❌ Failed to update database."
    exit 1
fi

echo "✅ Database updated."
