-- Your SQL goes here
CREATE TABLE work (
  work_id BIGSERIAL PRIMARY KEY,
  task_id BIGSERIAL,
  start_time BIGINT NOT NULL,
  duration BIGINT NOT NULL,
  progress DOUBLE PRECISION NOT NULL,
  finished BOOLEAN NOT NULL,
  music BOOLEAN,
  interruptions BIGINT,
  noise DOUBLE PRECISION,
  meetings BIGINT,
  breaks BIGINT
)
