CREATE TABLE IF NOT EXISTS orders (
  id TEXT PRIMARY KEY,
  token_id TEXT NOT NULL,
  size REAL NOT NULL,
  side TEXT NOT NULL,
  sl REAL,
  tp REAL,
  status TEXT NOT NULL,
  trigger_reason TEXT,
  trigger_price REAL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  executed_at TEXT
);
