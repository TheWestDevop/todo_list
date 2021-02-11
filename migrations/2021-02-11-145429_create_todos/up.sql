-- Your SQL goes here

CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  isCompleted BOOLEAN NOT NULL DEFAULT false,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
)