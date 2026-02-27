use crate::app_state::SharedState;
use crate::errors::AppError;
use crate::farms::models::{CreateFarm, Farm, FarmList};

pub async fn get_farms(state: SharedState) -> Result<FarmList, AppError> {
    let query = "SELECT id, name, location, created_at FROM farms ORDER BY created_at DESC";
    let farms = sqlx::query_as::<_, Farm>(query)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(FarmList { farms })
}

pub async fn create_farm(
    state: SharedState,
    farm_data: CreateFarm,
) -> Result<Farm, AppError> {
    let farm = Farm::new(farm_data);
    let query = "INSERT INTO farms (id, name, location) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(farm.id)
        .bind(&farm.name)
        .bind(&farm.location)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(farm)
}