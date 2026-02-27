CREATE TABLE inventory_items (
  id UUID PRIMARY KEY,
  farm_id UUID NOT NULL REFERENCES farms(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  quantity BIGINT NOT NULL DEFAULT 0,
  low_threshold BIGINT NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX inventory_items_farm_id_idx ON inventory_items(farm_id);
