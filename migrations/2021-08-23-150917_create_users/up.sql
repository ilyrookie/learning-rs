-- Your SQL goes here

CREATE table users (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT FALSE
)