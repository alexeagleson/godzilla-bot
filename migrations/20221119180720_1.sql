-- Add migration script here
CREATE TABLE films (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    year INTEGER NOT NULL,
    wikipedia TEXT NOT NULL
);
-- Add migration script here
CREATE TABLE monsters (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL COLLATE NOCASE,
    alternate_names TEXT NOT NULL COLLATE NOCASE,
    wikipedia TEXT,
    wikizilla TEXT NOT NULL,
    description TEXT NOT NULL
);
-- Add migration script here
CREATE TABLE watched (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    film_id INTEGER NOT NULL,
    -- watch_date Timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY(film_id) REFERENCES films(id)
);
-- Add migration script here
CREATE TABLE monsters_by_film (
    -- id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    monster_id INTEGER NOT NULL,
    film_id INTEGER NOT NULL,
    FOREIGN KEY(film_id) REFERENCES films(id),
    FOREIGN KEY(monster_id) REFERENCES monsters(id),
    PRIMARY KEY (monster_id, film_id)
);