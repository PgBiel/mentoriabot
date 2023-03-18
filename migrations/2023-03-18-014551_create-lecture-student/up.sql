CREATE TABLE lecture_students (
  lecture_id BIGINT REFERENCES lectures (id),
  user_id VARCHAR REFERENCES users (discord_id),
  PRIMARY KEY (user_id, lecture_id)
);
