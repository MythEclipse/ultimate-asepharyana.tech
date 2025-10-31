-- Seed data untuk chat rooms dan messages
-- File ini akan dijalankan setelah tabel dibuat
-- Safe version: Check if tables exist before inserting

-- Insert default chat rooms jika belum ada (only if table exists)
INSERT IGNORE INTO chat_rooms (id, name, description, created_by, created_at)
SELECT * FROM (
    SELECT '00000000-0000-0000-0000-000000000001' as id, 'General' as name, 'General discussion room for everyone' as description, 'system' as created_by, NOW() as created_at
    UNION ALL
    SELECT '00000000-0000-0000-0000-000000000002', 'Tech Talk', 'Discuss technology, programming, and development', 'system', NOW()
    UNION ALL
    SELECT '00000000-0000-0000-0000-000000000003', 'Random', 'Random chat and off-topic discussions', 'system', NOW()
) AS tmp
WHERE EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = DATABASE() AND table_name = 'chat_rooms');

-- Insert welcome messages untuk setiap room (only if table exists)
INSERT IGNORE INTO chat_messages (id, room_id, user_id, user_name, content, created_at)
SELECT * FROM (
    SELECT '00000000-0000-0000-0000-000000000001' as id, '00000000-0000-0000-0000-000000000001' as room_id, 'system' as user_id, 'System' as user_name, 'Welcome to the General chat room! Feel free to discuss anything here.' as content, NOW() as created_at
    UNION ALL
    SELECT '00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'system', 'System', 'Welcome to Tech Talk! Share your knowledge and learn from others.', NOW()
    UNION ALL
    SELECT '00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000003', 'system', 'System', 'Welcome to Random! This is a place for casual conversations.', NOW()
) AS tmp
WHERE EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = DATABASE() AND table_name = 'chat_messages');
