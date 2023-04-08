CREATE TABLE IF NOT EXISTS tiers (
  id            SERIAL PRIMARY KEY,
  title         TEXT NOT NULL,
  image         TEXT DEFAULT NULL,
  tier          VARCHAR(10) NOT NULL,
  kind          VARCHAR(50) DEFAULT 'normal',
  updated_at    TIMESTAMP DEFAULT NULL,
  deleted_at    TIMESTAMP DEFAULT NULL,
  game          VARCHAR(256) NOT NULL,
  x             INTEGER DEFAULT NULL,
  user_id       VARCHAR(256) DEFAULT NULL,
  list_id       VARCHAR(256) DEFAULT NULL,
  role          VARCHAR(256) DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS colors(
  id SERIAL PRIMARY KEY,
  kind VARCHAR(50) DEFAULT 'normal',
  start_color VARCHAR(256) DEFAULT NULL,
  end_color VARCHAR(256) DEFAULT NULL,
  game VARCHAR(256) NOT NULL
);

CREATE TABLE IF NOT EXISTS lists(
  id            SERIAL PRIMARY KEY,
  title         VARCHAR(256),
  user_id       VARCHAR(256),
  game          VARCHAR(256),
  created_at    TIMESTAMP DEFAULT NULL,
  updated_at    TIMESTAMP DEFAULT NULL
);