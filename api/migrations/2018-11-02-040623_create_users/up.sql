-- Your SQL goes here
CREATE TABLE users (
  user_id BIGSERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL
)
