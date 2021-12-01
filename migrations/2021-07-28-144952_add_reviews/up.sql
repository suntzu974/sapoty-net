-- Your SQL goes here
CREATE TABLE reviews (
  id SERIAL NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  original TEXT NOT NULL,
  thumbnail TEXT NOT NULL,
  web TEXT NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL
);