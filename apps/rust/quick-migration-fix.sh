#!/bin/bash
# Quick fix for migration errors - Run this on VPS

set -e

echo "=== Quick Migration Fix ==="
echo ""

cd "$(dirname "$0")"

# Load .env
if [ -f ".env" ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Parse DATABASE_URL
if [[ $DATABASE_URL =~ mysql://([^:]+):([^@]+)@([^:/]+)(:([0-9]+))?/(.+) ]]; then
    DB_USER="${BASH_REMATCH[1]}"
    DB_PASS="${BASH_REMATCH[2]}"
    DB_HOST="${BASH_REMATCH[3]}"
    DB_PORT="${BASH_REMATCH[5]:-3306}"
    DB_NAME="${BASH_REMATCH[6]}"

    echo "Database: $DB_NAME @ $DB_HOST:$DB_PORT"
    echo "User: $DB_USER"
    echo ""

    # Step 1: Ensure tables exist
    echo "Step 1: Ensuring all chat tables exist..."
    mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASS" "$DB_NAME" < ensure-chat-tables.sql
    echo "✓ Tables verified"
    echo ""

    # Step 2: Clear migration history
    echo "Step 2: Clearing migration history..."
    mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASS" "$DB_NAME" < fix-migrations.sql
    echo "✓ Migration history cleared"
    echo ""

    # Step 3: Restart app
    echo "Step 3: Restarting Rust app..."
    pm2 restart 3
    echo "✓ App restarted"
    echo ""

    # Step 4: Check status
    echo "Checking app status in 3 seconds..."
    sleep 3
    pm2 logs 3 --lines 20 --nostream

    echo ""
    echo "✓ Migration fix completed!"
    echo ""
    echo "Monitor logs with: pm2 logs 3"else
    echo "Error: Invalid DATABASE_URL format"
    exit 1
fi
