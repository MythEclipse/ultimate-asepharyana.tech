-- Fix migration history by removing non-existent migrations
-- This script removes migration records that don't have corresponding migration files

DELETE FROM _sqlx_migrations
WHERE version NOT IN (
    20251031162909,
    20251031180000,
    20251031190000,
    20251101013643
);

-- Show remaining migrations
SELECT * FROM _sqlx_migrations ORDER BY version;
