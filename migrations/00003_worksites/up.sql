-- Your SQL goes here
CREATE TABLE worksites (
    id SERIAL PRIMARY KEY,
    client_id INT NOT NULL,
    worksite jsonb NOT NULL,
    created_at TIMESTAMP NOT NULL,
    edited_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP NULL,
    FOREIGN KEY (client_id) REFERENCES clients(id)
);