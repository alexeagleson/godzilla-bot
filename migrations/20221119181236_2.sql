-- Add migration script here
-- Add migration script here
CREATE TABLE films (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    year INTEGER NOT NULL,
    monsters TEXT NOT NULL
)