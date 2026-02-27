use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Farm {
    pub id: Uuid,
    pub name: String,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFarm {
    pub name: String,
    pub location: Option<String>,
}

#[derive(Serialize)]
pub struct FarmList {
    pub farms: Vec<Farm>,
}

impl Farm {
    pub fn new(data: CreateFarm) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: data.name,
            location: data.location,
            created_at: Utc::now(),
        }
    }
}

