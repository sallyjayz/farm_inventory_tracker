use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct StockMovement {
    pub id: Uuid,
    pub item_id: Uuid,
    pub direction: String,
    pub quantity: i64,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct StockChange {
    pub quantity: i64,
    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct StockMovementList {
    pub stock_movements: Vec<StockMovement>,
}

impl StockMovement {
    pub fn new(
        item_id: Uuid,
        direction: String,
        quantity: i64,
        note: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            item_id,
            direction,
            quantity,
            note,
            created_at: Utc::now(),
        }
    }
}