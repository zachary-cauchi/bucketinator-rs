-- Your SQL goes here
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
)