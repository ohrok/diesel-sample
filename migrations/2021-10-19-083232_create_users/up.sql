-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  username TEXT NOT NULL UNIQUE
)