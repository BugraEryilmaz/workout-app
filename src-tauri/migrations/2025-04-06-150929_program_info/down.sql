-- This file should undo anything in `up.sql`
ALTER TABLE programs
  DROP COLUMN info;
ALTER TABLE programs
  DROP COLUMN created_at;