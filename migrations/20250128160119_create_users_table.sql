CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    totp_secret VARCHAR(255),
    role_level INT NOT NULL DEFAULT 1,
    tier_level INT NOT NULL DEFAULT 1,
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE,
    disabled BOOLEAN NOT NULL DEFAULT FALSE,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'active', 'disabled'
    verification_code TEXT, -- nullable, only used for pending users
    verification_expires_at TIMESTAMP WITH TIME ZONE, -- nullable, only used for pending users
    CONSTRAINT unique_username UNIQUE (username)
);

-- Insert the example 'user' into the users table with a conflict check for username
INSERT INTO users (username, email, password_hash, role_level, status)
VALUES 
    ('user', 'user@test.com', '$argon2i$v=19$m=16,t=2,p=1$ZE1qUWd0U21vUUlIM0ltaQ$dowBmjU4oHtoPd355dXypQ', 1, 'active')
ON CONFLICT (username) DO NOTHING;

-- Insert the example 'admin' into the users table with a conflict check for username
INSERT INTO users (username, email, password_hash, role_level, status)
VALUES 
    ('admin', 'admin@test.com', '$argon2i$v=19$m=16,t=2,p=1$ZE1qUWd0U21vUUlIM0ltaQ$dowBmjU4oHtoPd355dXypQ', 2, 'active')
ON CONFLICT (username) DO NOTHING;
