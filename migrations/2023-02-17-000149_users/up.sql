-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email TEXT NOT NULL unique,
  password TEXT NOT NULL
)