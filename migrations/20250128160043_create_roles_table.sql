-- Create the roles table
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    level INT NOT NULL,
    role VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE,  -- Default to the current date
    CONSTRAINT unique_role UNIQUE (role)  -- Add a unique constraint to the 'role' column
);

-- Insert a role into the roles table (this assumes role_id=1 is for 'user')
INSERT INTO roles (level, role, name, description)
VALUES (1, 'user', 'User', 'A regular user with basic access.')
ON CONFLICT (role) DO NOTHING;  -- Prevent duplicate insertions if role already exists

-- Insert a role into the roles table (this assumes role_id=2 is for 'admin')
INSERT INTO roles (level, role, name, description)
VALUES (2, 'admin', 'Administrator', 'An administrator.')
ON CONFLICT (role) DO NOTHING;  -- Prevent duplicate insertions if role already exists
