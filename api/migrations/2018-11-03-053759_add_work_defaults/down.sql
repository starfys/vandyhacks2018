-- This file should undo anything in `up.sql`
ALTER TABLE work ALTER COLUMN duration DROP DEFAULT;
ALTER TABLE work ALTER COLUMN progress DROP DEFAULT;
ALTER TABLE work ALTER COLUMN finished DROP DEFAULT;
