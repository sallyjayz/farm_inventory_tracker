-- Add migration script here
CREATE TABLE stock_movements (
  id UUID PRIMARY KEY,
  item_id UUID NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,
  direction TEXT NOT NULL CHECK (direction IN ('IN', 'OUT')),
  quantity BIGINT NOT NULL CHECK (quantity > 0),
  note TEXT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX stock_movements_item_id_idx ON stock_movements(item_id);