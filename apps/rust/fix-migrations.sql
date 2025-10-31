-- Fix migration history - Force clean state
-- This script completely resets migration state

-- Step 1: Show current migration state
SELECT 'Current migration state:' as info;
SELECT version, description, success, checksum, execution_time
FROM _sqlx_migrations
ORDER BY version;

-- Step 2: Drop and recreate _sqlx_migrations table for complete reset
DROP TABLE IF EXISTS _sqlx_migrations;

CREATE TABLE _sqlx_migrations (
    version BIGINT NOT NULL PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);

-- Step 3: Verify clean state
SELECT 'Migration table reset - completely clean:' as info;
SELECT COUNT(*) as remaining_records FROM _sqlx_migrations;
