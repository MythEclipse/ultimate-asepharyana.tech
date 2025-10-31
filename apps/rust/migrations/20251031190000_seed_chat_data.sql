-- Seed data untuk chat rooms dan messages
-- File ini akan dijalankan setelah tabel dibuat

-- Insert default chat rooms jika belum ada
INSERT IGNORE INTO chat_rooms (id, name, description, created_by, created_at) VALUES
('00000000-0000-0000-0000-000000000001', 'General', 'General discussion room for everyone', 'system', NOW()),
('00000000-0000-0000-0000-000000000002', 'Tech Talk', 'Discuss technology, programming, and development', 'system', NOW()),
('00000000-0000-0000-0000-000000000003', 'Random', 'Random chat and off-topic discussions', 'system', NOW());

-- Insert welcome messages untuk setiap room
INSERT IGNORE INTO chat_messages (id, room_id, user_id, user_name, content, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'system', 'System', 'Welcome to the General chat room! Feel free to discuss anything here.', NOW()),
('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'system', 'System', 'Welcome to Tech Talk! Share your knowledge and learn from others.', NOW()),
('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000003', 'system', 'System', 'Welcome to Random! This is a place for casual conversations.', NOW());
