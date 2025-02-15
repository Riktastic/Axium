-- Create the tiers table
CREATE TABLE IF NOT EXISTS tiers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    level INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    requests_per_day INT NOT NULL,
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE,  -- Default to the current date
    CONSTRAINT unique_name UNIQUE (name)  -- Add a unique constraint to the 'role' column
);

INSERT INTO tiers (level, name, description, requests_per_day)
VALUES (1, 'Low', 'Lowest amount of requests.', 1000)
ON CONFLICT (name) DO NOTHING;  -- Prevent duplicate insertions if role already exists

INSERT INTO tiers (level, name, description, requests_per_day)
VALUES (2, 'Medium', 'Medium amount of requests.',  5000)
ON CONFLICT (name) DO NOTHING;  -- Prevent duplicate insertions if role already exists

INSERT INTO tiers (level, name, description, requests_per_day)
VALUES (3, 'Max', 'Max amount of requests.',  10000)
ON CONFLICT (name) DO NOTHING;  -- Prevent duplicate insertions if role already exists
