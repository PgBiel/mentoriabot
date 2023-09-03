-- Represents sessions with the teachers
DROP TABLE IF EXISTS lecture;
DROP TABLE IF EXISTS session_students;

CREATE TABLE sessions (
  id BIGSERIAL PRIMARY KEY,
  teacher_id BIGSERIAL NOT NULL REFERENCES teachers (id),
  student_id VARCHAR NOT NULL REFERENCES users (discord_id),
  availability_id BIGINT NOT NULL REFERENCES availability (id),
  summary TEXT,
  notified BOOLEAN NOT NULL,
  meet_id VARCHAR,
  calendar_event_id VARCHAR,
  start_at timestamp with time zone NOT NULL,
  end_at timestamp with time zone NOT NULL,
  CHECK (start_at < end_at)
);
