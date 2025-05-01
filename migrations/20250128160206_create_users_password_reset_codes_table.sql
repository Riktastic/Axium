-- Add migration script here
CREATE TABLE users_password_reset_codes (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    code VARCHAR(64) PRIMARY KEY,
    expires_at TIMESTAMP NOT NULL
);