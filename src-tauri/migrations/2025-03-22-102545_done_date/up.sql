-- Your SQL goes here
-- Add done_date column to workouts table and set it to the completion date of the day
ALTER TABLE workouts ADD COLUMN done_date DATE;

-- Get the completion date from days table for each workout
UPDATE workouts
SET done_date = (SELECT complete_date FROM days WHERE days.id = workouts.day_id);

-- Update the done_date column to be today for all days that are not completed
UPDATE workouts
SET done_date = CURRENT_DATE
WHERE done_date IS NULL AND done = true;
