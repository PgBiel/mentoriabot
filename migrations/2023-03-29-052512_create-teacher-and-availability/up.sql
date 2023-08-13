-- Your SQL goes here
CREATE TABLE teachers (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR UNIQUE NOT NULL,
  specialty VARCHAR NOT NULL,
  applied_at timestamp with time zone,
  bio VARCHAR,
  course_info VARCHAR,
  company VARCHAR,
  company_role VARCHAR,
  whatsapp VARCHAR,
  linkedin VARCHAR,
  comment_general VARCHAR,
  comment_experience VARCHAR
);

CREATE TABLE availability (
  id BIGSERIAL PRIMARY KEY,
  teacher_id BIGSERIAL NOT NULL REFERENCES teachers (id),
  weekday SMALLINT NOT NULL CHECK (weekday >= 0 AND weekday <= 6),
  time_start time NOT NULL,
  expired BOOLEAN NOT NULL DEFAULT false,
  duration SMALLINT NOT NULL CHECK (duration >= 0)
);
