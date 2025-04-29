ALTER TABLE users
    ADD COLUMN first_name VARCHAR(50),
    ADD COLUMN last_name VARCHAR(50),
    ADD COLUMN country_code CHAR(2),
    ADD COLUMN language_code CHAR(5),
    ADD COLUMN birthday DATE,
    ADD COLUMN description TEXT,
    ADD COLUMN profile_picture_url TEXT;