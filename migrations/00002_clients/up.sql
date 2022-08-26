-- Your SQL goes here
CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address jsonb NOT NULL,
    interlocutors jsonb NOT NULL,
    created_at TIMESTAMP NOT NULL,
    edited_at TIMESTAMP NOT NULL
);