CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    totp_secret VARCHAR(255),
    role_id INT NOT NULL DEFAULT 1 REFERENCES roles(id),  -- Default role_id is set to 1
    CONSTRAINT unique_username UNIQUE (username)  -- Ensure that username is unique
);

-- Insert the example 'user' into the users table with a conflict check for username
INSERT INTO users (username, email, password_hash, role_id)
VALUES 
    ('user', 'user@test.com', '$argon2i$v=19$m=16,t=2,p=1$ZE1qUWd0U21vUUlIM0ltaQ$dowBmjU4oHtoPd355dXypQ', 1)
ON CONFLICT (username) DO NOTHING;  -- Prevent duplicate insertions if username already exists

-- Insert the example 'admin' into the users table with a conflict check for username
INSERT INTO users (username, email, password_hash, role_id)
VALUES 
    ('admin', 'admin@test.com', '$argon2i$v=19$m=16,t=2,p=1$ZE1qUWd0U21vUUlIM0ltaQ$dowBmjU4oHtoPd355dXypQ', 2)
ON CONFLICT (username) DO NOTHING;  -- Prevent duplicate insertions if username already exists