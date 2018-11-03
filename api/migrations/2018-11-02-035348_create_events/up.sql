-- Your SQL goes here
CREATE TABLE events (
  event_id BIGSERIAL PRIMARY KEY,
  user_id BIGSERIAL,
  name VARCHAR NOT NULL,
  description TEXT,
  begin_timestamp VARCHAR NOT NULL,
  end_timestamp VARCHAR NOT NULL,
  recurrence VARCHAR NOT NULL,
  importance BIGINT NOT NULL
)
