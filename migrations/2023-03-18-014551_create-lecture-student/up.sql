CREATE TABLE lecture_students (
  lecture_id BIGINT NOT NULL REFERENCES lectures (id),
  user_id VARCHAR NOT NULL REFERENCES users (discord_id),
  PRIMARY KEY (user_id, lecture_id)
);
