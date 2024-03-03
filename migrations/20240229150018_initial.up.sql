-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    age INT NOT NULL
);
