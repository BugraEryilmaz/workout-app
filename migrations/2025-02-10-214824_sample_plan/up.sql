-- Your SQL goes here
INSERT INTO programs (title, active) VALUES
('30 Days of Yoga', TRUE),
('10 Days of Abs', FALSE);

INSERT INTO days (id, program_id, done, complete_date) VALUES
('5', '1', TRUE, '2025-02-10'),
('6', '1', TRUE, '2025-02-11'),
('7', '1', TRUE, '2025-02-13'),
('8', '1', FALSE, NULL),
('9', '1', FALSE, NULL),
('10', '1', FALSE, NULL),
('1','2', FALSE, NULL),
('2','2', FALSE, NULL),
('3','2', FALSE, NULL),
('4','2', FALSE, NULL);

INSERT INTO workouts (link, title, duration, day_id, done) VALUES
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 1', 1274, '5', TRUE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 1', 638, '5', TRUE),
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 2', 1274, '6', TRUE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 2', 638, '6', TRUE),
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 3', 1274, '7', TRUE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 3', 638, '7', TRUE),
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 4', 1274, '8', TRUE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 4', 638, '8', FALSE),
-- ('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 5', 1274, '9', FALSE),
-- ('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 5', 638, '9', FALSE),
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 6', 1274, '10', FALSE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 6', 638, '10', FALSE),
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 7', 1274, '1', FALSE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 7', 638, '1', FALSE),
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 8', 1274, '2', FALSE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 8', 638, '2', FALSE),
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 9', 1274, '3', FALSE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 9', 638, '3', FALSE),
('https://youtu.be/HIfQ6botXm4?si=M0MLcAPAHFXmQKm5', '20 min hiit 10', 1274, '4', FALSE),
('https://youtu.be/XkPTWfH5h70?si=kyWQ-KHagPMYKMwS', '10 min abs 10', 638, '4', FALSE);
