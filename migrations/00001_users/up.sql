-- Your SQL goes here
CREATE TABLE authorizations (
  id SERIAL PRIMARY KEY,
  level VARCHAR NOT NULL
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    authorization_id INT NOT NULL,
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    FOREIGN KEY (authorization_id) REFERENCES authorizations(id)
);

INSERT INTO authorizations (level) VALUES ('editeur');
INSERT INTO authorizations (level) VALUES ('administrateur');
INSERT INTO authorizations (level) VALUES ('developpeur');