-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    discord_userid BIGINT NOT NULL,
    bio TEXT
);
