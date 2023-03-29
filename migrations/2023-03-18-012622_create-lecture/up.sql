CREATE TABLE lectures (
  id BIGSERIAL PRIMARY KEY,
  teacher_id VARCHAR NOT NULL REFERENCES users (discord_id),
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  notified BOOLEAN NOT NULL,
  start_at timestamp with time zone NOT NULL,
  end_at timestamp with time zone NOT NULL
);
