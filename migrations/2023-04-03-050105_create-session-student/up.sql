-- Your SQL goes here
CREATE TABLE session_students (
  session_id BIGINT NOT NULL REFERENCES sessions (id),
  user_id VARCHAR NOT NULL REFERENCES users (discord_id),
  PRIMARY KEY (user_id, session_id)
);
