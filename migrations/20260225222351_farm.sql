-- Add migration script here
CREATE TABLE farms (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  location TEXT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);