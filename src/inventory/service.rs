use sqlx::QueryBuilder;
use uuid::Uuid;
use crate::inventory::models::{
    InventoryItem, InventoryItemList, CreateInventoryItem,
};
use crate::{app_state::SharedState, errors::AppError};

// Create an item
pub async fn create_inventory_item(
    state: SharedState,
    payload: CreateInventoryItem,
) -> Result<InventoryItem, AppError> {
    let item = InventoryItem::new(
        payload.farm_id,
        payload.name,
        payload.quantity,
        payload.low_threshold,
    );

    sqlx::query(
        "INSERT INTO inventory_items (id, farm_id, name, quantity, low_threshold, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(item.id)
    .bind(item.farm_id)
    .bind(&item.name)
    .bind(item.quantity)
    .bind(item.low_threshold)
    .bind(item.created_at)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(item)
}

pub async fn get_item_by_id(
    state: SharedState,
    item_id: String,
) -> Result<InventoryItem, AppError> {
    let item_id =
        Uuid::parse_str(&item_id).map_err(|e| AppError::ParsingError(e.to_string()))?;
    let item = sqlx::query_as::<_, InventoryItem>(
        "SELECT id, farm_id, name, quantity, low_threshold, created_at FROM inventory_items WHERE id = $1",
    )
    .bind(item_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(item)
}

pub async fn get_items(
    state: SharedState,
    farm_id: Option<Uuid>,
) -> Result<InventoryItemList, AppError> {
    let mut builder = QueryBuilder::new(
        "SELECT id, farm_id, name, quantity, low_threshold, created_at FROM inventory_items",
    );


    if let Some(fid) = farm_id {
        builder.push(" WHERE farm_id = ");
        builder.push_bind(fid);
        builder.push(" ORDER BY created_at DESC");
    }

    let items = builder
        .build_query_as::<InventoryItem>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(InventoryItemList { items })
}