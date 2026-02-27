use uuid::Uuid;
use crate::inventory::models::InventoryItem;
use crate::stock_movement::models::{
    StockMovement, StockMovementList, StockChange,
};
use crate::{app_state::SharedState, errors::AppError};

// async fn create_stock_movement(
//     state: SharedState,
//     payload: CreateStockMovement,
// ) -> Result<StockMovement, AppError> {
//     let stock_movement = StockMovement::new(
//         payload.item_id,
//         payload.direction,
//         payload.quantity,
//         payload.note,
//     );

//     sqlx::query(
//         "INSERT INTO stock_movements (id, item_id, direction, quantity, note, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
//     )
//     .bind(stock_movement.id)
//     .bind(stock_movement.item_id)
//     .bind(&stock_movement.direction)
//     .bind(stock_movement.quantity)
//     .bind(&stock_movement.note)
//     .bind(stock_movement.created_at)
//     .execute(&state.db_pool)
//     .await
//     .map_err(|e| AppError::DatabaseError(e.to_string()))?;

//     Ok(stock_movement)
// }

pub async fn add_stock(
    state: SharedState,
    item_id: Uuid,
    payload: StockChange,
) -> Result<InventoryItem, AppError> {
    change_stock(state, item_id, "IN", payload.quantity, payload.note).await
}

pub async fn remove_stock(
    state: SharedState,
    item_id: Uuid,
    payload: StockChange,
) -> Result<InventoryItem, AppError> {
    change_stock(state, item_id, "OUT", payload.quantity, payload.note).await
}

pub async fn change_stock(
    state: SharedState,
    item_id: Uuid,
    direction: &str,
    quantity_change: i64,
    note: Option<String>,
) -> Result<InventoryItem, AppError> {

    if quantity_change <= 0 {
        return Err(AppError::UnProcessableEntity {
            field: "quantity_change".into(),
            message: "must be > 0".into(),
        });
    }

    let item_selected = sqlx::query_as::<_, InventoryItem>(
        "SELECT * FROM inventory_items WHERE id = $1",
    )
    .bind(item_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    

    let new_qty = if direction == "IN" {
        item_selected.quantity + quantity_change
    } else {
        let candidate = item_selected.quantity - quantity_change; 
        if candidate < 0 {
            return Err(AppError::UnProcessableEntity {
                field: "quantity_change".into(),
                message: "insufficient stock".into(),
            });
        }
        candidate      
    }; 

    let item_updated = sqlx::query_as::<_, InventoryItem>(
        "UPDATE inventory_items SET quantity = $1 WHERE id = $2 RETURNING *",
    )
    .bind(new_qty)
    .bind(item_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let stock_movement = StockMovement::new(
        item_id,
        direction.to_string(),
        quantity_change,
        note.clone(),
    );

    sqlx::query(
        "INSERT INTO stock_movements (id, item_id, direction, quantity, note, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(stock_movement.id)
    .bind(item_id)
    .bind(direction)
    .bind(quantity_change)
    .bind(note)
    .bind(stock_movement.created_at)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(item_updated)

    
}

pub async fn get_stock_movements(state: SharedState) -> Result<StockMovementList, AppError> {
    let query = "SELECT * FROM stock_movements ORDER BY created_at DESC";
    let stock_movements = sqlx::query_as::<_, StockMovement>(query)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(StockMovementList { stock_movements })
}