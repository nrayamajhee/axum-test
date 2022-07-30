CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id        UUID    PRIMARY KEY DEFAULT uuid_generate_v4(),
  username  TEXT    NOT NULL    UNIQUE,
  password  TEXT    NOT NULL,
  name      TEXT    NOT NULL,
  email     TEXT    NOT NULL,
  bio       TEXT    NOT NULL    DEFAULT '',
  verified  BOOLEAN NOT NULL    DEFAULT false
);

CREATE TABLE IF NOT EXISTS posts (
  id        UUID    PRIMARY KEY DEFAULT uuid_generate_v4(),
  author_id UUID    NOT NULL    REFERENCES users,
  title     TEXT    NOT NULL,
  slug      TEXT    NOT NULL    UNIQUE,
  tags      TEXT[]  NOT NULL    DEFAULT '{}',
  body      TEXT    NOT NULL    DEFAULT ''
)
