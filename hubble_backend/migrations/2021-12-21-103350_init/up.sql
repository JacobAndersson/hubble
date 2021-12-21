-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR PRIMARY KEY,
  rating INTEGER,
  UNIQUE(id)
);

CREATE TABLE games (
  id VARCHAR NOT NULL PRIMARY KEY,
  opening_id VARCHAR,
  moves JSONB NOT NULL,
  scores JSONB NOT NULL,

  white VARCHAR NOT NULL,
  black VARCHAR NOT NULL,
  white_rating INTEGER,
  black_rating INTEGER,

  winner VARCHAR,
  UNIQUE(id)
);
