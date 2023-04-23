-- Your SQL goes here
CREATE TABLE teachers (
  user_id VARCHAR PRIMARY KEY REFERENCES users (discord_id),
  email VARCHAR UNIQUE,
  specialty VARCHAR NOT NULL,
  company VARCHAR,
  company_role VARCHAR
);

CREATE TABLE availability (
  id BIGSERIAL PRIMARY KEY,
  teacher_id VARCHAR NOT NULL REFERENCES teachers (user_id),
  weekday SMALLINT NOT NULL CHECK (weekday >= 0 AND weekday <= 6),
  time_start time NOT NULL,
  time_end time NOT NULL,
  CHECK (time_start < time_end)
);
