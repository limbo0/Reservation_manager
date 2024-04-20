-- Your SQL goes here
CREATE TABLE resv (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    contact TEXT NOT NULL,
    seating VARCHAR NOT NULL,
    advance BOOL NOT NULL,
    confirmed BOOL NOT NULL
);
