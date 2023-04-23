-- Represents sessions with the teachers
DROP TABLE IF EXISTS lecture;
DROP TABLE IF EXISTS session_students;

CREATE TABLE sessions (
  id BIGSERIAL PRIMARY KEY,
  teacher_id VARCHAR NOT NULL REFERENCES teachers (user_id),
  student_id VARCHAR NOT NULL REFERENCES users (discord_id),
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  notified BOOLEAN NOT NULL,
  availability_id BIGINT REFERENCES availability (id) ON DELETE SET NULL,
  start_at timestamp with time zone NOT NULL,
  end_at timestamp with time zone NOT NULL
);
