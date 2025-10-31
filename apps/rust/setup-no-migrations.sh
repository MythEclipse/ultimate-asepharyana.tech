#!/bin/bash
# One-time database setup - No migrations needed

cd "$(dirname "$0")"

echo "=== Database Setup (No Migrations) ==="
echo ""

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
    echo ""
    
    echo "Creating all tables..."
    mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASS" "$DB_NAME" < setup-database.sql
    
    echo ""
    echo "✓ Database setup completed!"
    echo ""
    echo "Restarting Rust app..."
    pm2 restart 3
    
    sleep 3
    echo ""
    echo "App logs:"
    pm2 logs 3 --lines 30 --nostream
    
else
    echo "Error: Invalid DATABASE_URL"
    exit 1
fi
