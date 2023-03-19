-- Your SQL goes here
CREATE TABLE users (
    discord_id VARCHAR PRIMARY KEY CHECK(discord_id ~ '^\d{1,20}$'),
    name VARCHAR NOT NULL,
    bio TEXT
);
