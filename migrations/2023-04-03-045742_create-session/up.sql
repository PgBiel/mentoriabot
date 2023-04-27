-- Represents sessions with the teachers
DROP TABLE IF EXISTS lecture;
DROP TABLE IF EXISTS session_students;

CREATE TABLE sessions (
  id BIGSERIAL PRIMARY KEY,
  teacher_id VARCHAR NOT NULL REFERENCES teachers (user_id),
  student_id VARCHAR NOT NULL REFERENCES users (discord_id),
  availability_id BIGINT NOT NULL REFERENCES availability (id),
  summary TEXT,
  notified BOOLEAN NOT NULL,
  start_at timestamp with time zone NOT NULL,
  end_at timestamp with time zone NOT NULL,
  CHECK (start_at < end_at)
);
