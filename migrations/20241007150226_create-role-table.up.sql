CREATE TABLE
    "roles" (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL
    );

INSERT INTO roles(name)
VALUES 
('admin'),
('user');