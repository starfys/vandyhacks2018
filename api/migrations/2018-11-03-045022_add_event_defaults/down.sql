-- This file should undo anything in `up.sql`
ALTER TABLE tasks ALTER COLUMN in_progress DROP DEFAULT;
ALTER TABLE tasks ALTER COLUMN progress DROP DEFAULT;
ALTER TABLE tasks ALTER COLUMN completed DROP DEFAULT;
