-- Your SQL goes here

CREATE TABLE files (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    path VARCHAR NOT NULL,
    mime_type_id INTEGER NOT NULL,
    size INTEGER NOT NULL
);
