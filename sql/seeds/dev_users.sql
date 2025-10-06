-- Development test users
-- Password for both: Ab13Cba46def79_

INSERT INTO users (id, email, password_hash, full_name, created_at, updated_at, is_active)
VALUES
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'admin@xforce.dev',
        '$argon2id$v=19$m=19456,t=2,p=1$qwertyuiopasdfghjklzxcvbnm$B1234567890abcdefghijklmnopqrstuvwxyz',
        'Admin User',
        NOW(),
        NOW(),
        true
    ),
    (
        'b1ffbc99-9c0b-4ef8-bb6d-6bb9bd380a22',
        'user@xforce.dev',
        '$argon2id$v=19$m=19456,t=2,p=1$qwertyuiopasdfghjklzxcvbnm$B1234567890abcdefghijklmnopqrstuvwxyz',
        'Test User',
        NOW(),
        NOW(),
        true
    )
ON CONFLICT (email) DO NOTHING;

-- Note: These are placeholder hashes. In production, use proper password hashing.
-- To generate real hashes, run the application and create users via the signup endpoint.
