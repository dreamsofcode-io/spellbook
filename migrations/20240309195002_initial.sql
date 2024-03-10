CREATE TABLE spell (
  id bigserial primary key,
  name VARCHAR,
  damage INT not null,
  created_at TIMESTAMPTZ NOT NULL default now(),
  updated_at TIMESTAMPTZ NOT NULL default now()
);
