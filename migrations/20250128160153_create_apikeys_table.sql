CREATE TABLE apikeys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key_hash VARCHAR(255) NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    description VARCHAR(255),
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE,  -- Default to the current date
    expiration_date DATE,
    disabled BOOLEAN NOT NULL DEFAULT FALSE,  -- Default to false
    access_read BOOLEAN NOT NULL DEFAULT TRUE,  -- Default to 
    access_modify BOOLEAN NOT NULL DEFAULT FALSE,  -- Default to false
    CONSTRAINT unique_key_hash UNIQUE (key_hash)  -- Add a unique constraint to the 'key_hash' column
);