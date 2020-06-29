CREATE TYPE platforms AS ENUM ('hackerone', 'bugcrowd', 'synack', 'intigriti', 'custom');
CREATE TYPE http_verbs AS ENUM ('GET', 'HEAD', 'POST', 'PUT', 'DELETE', 'CONNECT', 'OPTIONS', 'TRACE', 'PATCH');

CREATE TABLE IF NOT EXISTS programs (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  link TEXT NOT NULL,
  platform platforms NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  last_scanned_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS domains (
  id SERIAL PRIMARY KEY,
  host TEXT NOT NULL,
  is_in_scope BOOLEAN NOT NULL,
  are_subs_in_scope BOOLEAN NOT NULL,
  source TEXT NOT NULL,
  cname text,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  last_scanned_at TIMESTAMP,
  manually_reviewed BOOLEAN NOT NULL DEFAULT false,
  program_id INT REFERENCES programs (id)
);

CREATE TABLE IF NOT EXISTS ips (
  id SERIAL PRIMARY KEY,
  ip INET,
  last_scanned_at TIMESTAMP,
  program_id INT REFERENCES programs (id)
);

CREATE TABLE IF NOT EXISTS ip_domain_join (
  id SERIAL PRIMARY KEY,
  domain_id INT REFERENCES domains (id),
  ip_id INT REFERENCES ips (id)
);

CREATE TABLE IF NOT EXISTS ports (
  id SERIAL PRIMARY KEY,
  port INT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  manually_reviewed BOOLEAN NOT NULL DEFAULT false,
  ip_id INT REFERENCES ips (id)
);

CREATE TABLE IF NOT EXISTS endpoints (
  id SERIAL PRIMARY KEY,
  url TEXT NOT NULL,
  manually_reviewed BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  verbs http_verbs[] NOT NULL,
  domain_id INT REFERENCES domains (id),
  port_id INT REFERENCES ports(id)
);

CREATE TABLE IF NOT EXISTS parameters (
  id SERIAL PRIMARY KEY,
  type TEXT,
  parameter TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  endpoint_id INT REFERENCES endpoints (id)
);
