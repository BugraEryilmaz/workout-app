-- Your SQL goes here
CREATE TABLE IF NOT EXISTS programs (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(255) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    image VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS days (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    program_id INTEGER NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    complete_date DATE,
    day_number INTEGER,
    FOREIGN KEY (program_id) REFERENCES programs(id) ON DELETE CASCADE,
    UNIQUE (program_id, day_number)
);

CREATE TRIGGER IF NOT EXISTS auto_increment_trigger
    AFTER INSERT ON days
    WHEN new.day_number IS NULL
    BEGIN
        UPDATE days
        SET day_number = (SELECT IFNULL(MAX(day_number), 0) + 1 FROM days WHERE program_id = new.program_id)
        WHERE id = new.id;
    END;

CREATE TABLE IF NOT EXISTS workouts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    link VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    duration INT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    day_id INTEGER NOT NULL,
    FOREIGN KEY (day_id) REFERENCES days(id) ON DELETE CASCADE
);
