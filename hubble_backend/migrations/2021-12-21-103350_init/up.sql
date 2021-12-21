-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR PRIMARY KEY,
  rating INTEGER,
  UNIQUE(id)
);

CREATE TABLE matches (
  id VARCHAR NOT NULL PRIMARY KEY,
  player_id VARCHAR NOT NULL,
  opening_id VARCHAR NOT NULL,
  moves JSONB NOT NULL,
  scores JSONB NOT NULL,
  winner VARCHAR NOT NULL,
  player_rating INTEGER,
  oponnent_rating INTEGER,
  is_white BOOLEAN NOT NULL,
  UNIQUE(id)
);
