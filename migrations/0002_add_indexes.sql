CREATE INDEX IF NOT EXISTS idx_orders_token_status ON orders(token_id, status);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
