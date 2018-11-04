-- Your SQL goes here
ALTER TABLE tasks ALTER COLUMN in_progress SET DEFAULT false;
ALTER TABLE tasks ALTER COLUMN progress SET DEFAULT 0.0;
ALTER TABLE tasks ALTER COLUMN completed SET DEFAULT false;
