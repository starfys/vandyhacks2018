-- Your SQL goes here
CREATE TABLE tasks (
  task_id BIGSERIAL PRIMARY KEY,
  owner_id BIGSERIAL,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  created BIGINT NOT NULL,
  due BIGINT NOT NULL,
  importance DOUBLE PRECISION NOT NULL,
  in_progress BOOLEAN NOT NULL,
  progress DOUBLE PRECISION NOT NULL,
  completed BOOLEAN NOT NULL
)
