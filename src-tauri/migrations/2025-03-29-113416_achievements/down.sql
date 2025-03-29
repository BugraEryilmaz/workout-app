-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS achievements;

ALTER TABLE programs DROP COLUMN deleted;