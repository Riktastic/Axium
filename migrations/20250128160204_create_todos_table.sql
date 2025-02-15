CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),     -- Auto-incrementing primary key
    task TEXT NOT NULL,                   -- Task description, cannot be null
    description TEXT,                     -- Optional detailed description
    user_id UUID NOT NULL REFERENCES users(id),  -- Foreign key to link to users table
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE,  -- Default to the current date
    completion_date DATE,                 -- Date the task was completed
    completed BOOLEAN DEFAULT FALSE       -- Default to false
);