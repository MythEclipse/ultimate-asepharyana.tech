-- Fix migration history - Complete reset for clean state
-- This script safely cleans up migration state and allows fresh re-run

-- Step 1: Show current migration state
SELECT 'Current migration state:' as info;
SELECT version, description, success, checksum, execution_time
FROM _sqlx_migrations
ORDER BY version;

-- Step 2: Delete ALL migration records to force fresh re-run
-- This is safe because:
-- 1. Tables already exist (created with CREATE TABLE IF NOT EXISTS)
-- 2. Seed data uses INSERT IGNORE (won't duplicate)
-- 3. Allows sqlx to re-run migrations cleanly
DELETE FROM _sqlx_migrations;

-- Step 3: Show updated migration state
SELECT 'Migration table cleared - will be repopulated on next app start:' as info;
SELECT COUNT(*) as remaining_records FROM _sqlx_migrations;
