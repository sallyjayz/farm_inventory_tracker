use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, Serialize, FromRow)]
pub struct InventoryItem {
    pub id: Uuid,
    pub farm_id: Uuid,
    pub name: String,
    pub quantity: i64,
    pub low_threshold: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInventoryItem {
    pub farm_id: Uuid,
    pub name: String,
    pub quantity: i64,
    pub low_threshold: i64,
}

#[derive(Serialize)]
pub struct InventoryItemList {
    pub items: Vec<InventoryItem>,
}

impl InventoryItem {
    pub fn new(
        farm_id: Uuid,
        name: String,
        quantity: i64,
        low_threshold: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            farm_id,
            name,
            quantity,
            low_threshold,
            created_at: Utc::now(),
        }
    }
}





