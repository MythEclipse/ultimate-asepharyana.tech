#!/bin/bash
# Safe migration reset and re-run script for VPS

set -e

echo "=== Safe Migration Reset ==="
echo ""

# Load .env file
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

    # Confirm action
    read -p "This will reset migration history. Continue? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        echo "Aborted."
        exit 0
    fi

    echo ""
    echo "Step 1: Backing up current migration state..."

    # Create backup
    backup_file="migration_backup_$(date +%Y%m%d_%H%M%S).sql"
    mysqldump -h"$DB_HOST" -P"$DB_PORT" -u"$DB_USER" -p"$DB_PASS" "$DB_NAME" _sqlx_migrations > "$backup_file" 2>/dev/null || true

    # Show current state
    echo "Current migrations:"
    mysql -h"$DB_HOST" -P"$DB_PORT" -u"$DB_USER" -p"$DB_PASS" "$DB_NAME" -e "SELECT * FROM _sqlx_migrations ORDER BY version;"

    echo ""
    echo "Step 2: Cleaning migration history..."

    # Run fix-migrations.sql
    mysql -h"$DB_HOST" -P"$DB_PORT" -u"$DB_USER" -p"$DB_PASS" "$DB_NAME" < fix-migrations.sql

    echo ""
    echo "Step 3: Verifying migration state..."
    mysql -h"$DB_HOST" -P"$DB_PORT" -u"$DB_USER" -p"$DB_PASS" "$DB_NAME" -e "SELECT * FROM _sqlx_migrations ORDER BY version;"

    echo ""
    echo "✓ Migration history cleaned successfully!"
    echo ""
    echo "Backup saved to: $backup_file"
    echo ""
    echo "Next steps:"
    echo "1. Restart the Rust app: pm2 restart RustExpr"
    echo "2. Check logs: pm2 logs RustExpr"

else
    echo "Invalid DATABASE_URL format!"
    echo "Expected: mysql://user:pass@host:port/database"
    exit 1
fi
