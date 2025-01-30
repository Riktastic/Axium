CREATE TABLE todos (
    id SERIAL PRIMARY KEY,                -- Auto-incrementing primary key
    task TEXT NOT NULL,                   -- Task description, cannot be null
    description TEXT,                     -- Optional detailed description
    user_id INT NOT NULL REFERENCES users(id)  -- Foreign key to link to users table
);