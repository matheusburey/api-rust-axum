CREATE TABLE people (
  id serial PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  nick VARCHAR(32) NOT NULL,
  birth_date DATE NOT NULL,
  stack VARCHAR(32)[],
  CONSTRAINT unique_nick UNIQUE (nick)
);