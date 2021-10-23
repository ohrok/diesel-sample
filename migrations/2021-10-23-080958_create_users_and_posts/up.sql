-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  username TEXT NOT NULL UNIQUE
);

CREATE TABLE posts (
  id UUID PRIMARY KEY,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  user_id UUID NOT NULL REFERENCES users(id)
);