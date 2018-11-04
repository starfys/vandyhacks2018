-- This file should undo anything in `up.sql`
ALTER TABLE work RENAME COLUMN end_time TO duration;
