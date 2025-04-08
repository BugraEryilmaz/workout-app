-- Your SQL goes here
ALTER TABLE programs ADD COLUMN info TEXT NOT NULL DEFAULT '';
ALTER TABLE programs ADD COLUMN created_at DATE NOT NULL DEFAULT "1970-01-01";

UPDATE programs
SET created_at = CURRENT_DATE
WHERE created_at IS "1970-01-01";
