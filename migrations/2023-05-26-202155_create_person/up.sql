-- Your SQL goes here
CREATE TABLE IF NOT EXISTS person_diesel (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    age INT NOT NULL
);