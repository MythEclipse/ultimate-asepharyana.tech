#!/bin/bash
# Force reset migration - No questions asked

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

    echo "=== Force Migration Reset ==="
    echo "Database: $DB_NAME @ $DB_HOST:$DB_PORT"
    echo ""

    # Drop and recreate migration table for complete reset
    echo "Resetting migration table..."
    mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASS" "$DB_NAME" < fix-migrations.sql
    echo "✓ Migration table reset"    echo ""
    echo "Restarting Rust app..."
    pm2 restart 3

    echo ""
    echo "Waiting for app to start..."
    sleep 3

    echo ""
    echo "Recent logs:"
    pm2 logs 3 --lines 30 --nostream

    echo ""
    echo "Done! Check if migrations ran successfully above."

else
    echo "Error: Invalid DATABASE_URL"
    exit 1
fi
