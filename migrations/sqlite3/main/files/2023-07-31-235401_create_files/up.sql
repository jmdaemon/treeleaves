-- Your SQL goes here
CREATE TABLE files (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    mime_type_id INTEGER NOT NULL,
    size INTEGER NOT NULL
);
