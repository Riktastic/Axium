CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    totp_secret VARCHAR(255),
    role_level INT NOT NULL DEFAULT 1,  -- Default role_id is set to 1
    tier_level INT NOT NULL DEFAULT 1,  -- Default role_id is set to 1
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE,  -- Default to the current date
    disabled BOOLEAN NOT NULL DEFAULT FALSE,  -- Default to false
    CONSTRAINT unique_username UNIQUE (username)  -- Ensure that username is unique
);

-- Insert the example 'user' into the users table with a conflict check for username
INSERT INTO users (username, email, password_hash, role_level)
VALUES 
    ('user', 'user@test.com', '$argon2i$v=19$m=16,t=2,p=1$ZE1qUWd0U21vUUlIM0ltaQ$dowBmjU4oHtoPd355dXypQ', 1)
ON CONFLICT (username) DO NOTHING;

-- Insert the example 'admin' into the users table with a conflict check for username
INSERT INTO users (username, email, password_hash, role_level)
VALUES 
    ('admin', 'admin@test.com', '$argon2i$v=19$m=16,t=2,p=1$ZE1qUWd0U21vUUlIM0ltaQ$dowBmjU4oHtoPd355dXypQ', 2)
ON CONFLICT (username) DO NOTHING;
