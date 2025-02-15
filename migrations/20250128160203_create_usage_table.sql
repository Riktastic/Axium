CREATE TABLE usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    endpoint VARCHAR(255) NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE  -- Default to the current date
);