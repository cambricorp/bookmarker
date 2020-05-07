CREATE TABLE units (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  building TEXT NOT NULL,
  minerals INTEGER NOT NULL,
  gas INTEGER NOT NULL,
  supply INTEGER NOT NULL
);
