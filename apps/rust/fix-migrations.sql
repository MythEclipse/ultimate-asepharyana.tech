-- Fix migration history - Remove invalid/dirty migrations
-- This script safely cleans up migration state

-- Step 1: Show current migration state
SELECT 'Current migration state:' as info;
SELECT version, description, success, checksum, execution_time
FROM _sqlx_migrations
ORDER BY version;

-- Step 2: Delete migrations that don't exist in code
DELETE FROM _sqlx_migrations
WHERE version NOT IN (
    20251031162909,
    20251031180000,
    20251031190000,
    20251101013643
);

-- Step 3: Fix dirty migrations (set success = true)
UPDATE _sqlx_migrations
SET success = true
WHERE version IN (
    20251031162909,
    20251031180000,
    20251031190000,
    20251101013643
) AND success = false;

-- Step 4: Show updated migration state
SELECT 'Updated migration state:' as info;
SELECT version, description, success, checksum, execution_time
FROM _sqlx_migrations
ORDER BY version;
