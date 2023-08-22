-- Your SQL goes here

CREATE TABLE files (
    id BIGINT NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    path VARCHAR NOT NULL,
    mime_type_id INTEGER NOT NULL,
    size BIGINT NOT NULL
);
