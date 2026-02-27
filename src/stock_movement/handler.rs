use axum::extract::{Path, State};
use uuid::Uuid;

use crate::stock_movement::models::{StockChange};
use crate::stock_movement::service;
use crate::{app_state::SharedState, errors::AppError};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

// pub async fn stock_change_handler(
//     State(state): State<SharedState>,
//     Json(payload): Json<StockChange>,
// ) -> impl IntoResponse {
//     match service::change_stock(state, payload.item_id, &payload.direction, payload.quantity, payload.note).await {
//         Ok(item) => (StatusCode::OK, Json(item)).into_response(),
//         Err(e) => match e {
//             AppError::DatabaseError(_) => (
//                 StatusCode::NOT_FOUND,
//                 Json(json!({"error": "Item not found"})),
//             )
//                 .into_response(),
//             AppError::ParsingError(e) => (
//                 StatusCode::BAD_REQUEST,
//                 Json(json!({"error": e.to_string()})),
//             )
//                 .into_response(),
//             AppError::UnProcessableEntity { field, message } => (
//                 StatusCode::UNPROCESSABLE_ENTITY,
//                 Json(json!({ "field": field, "message": message })),
//             )
//                 .into_response(),
//             _ => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({"error": e.to_string()})),
//             )
//                 .into_response(),
//         },
//     }
// }


pub async fn add_stock_handler(
    State(state): State<SharedState>,
    Path(item_id): Path<Uuid>,
    Json(payload): Json<StockChange>,
) -> impl IntoResponse {
    match service::add_stock(state, item_id, payload).await {
        Ok(item) => (StatusCode::OK, Json(item)).into_response(),
        Err(e) => match e {
            AppError::DatabaseError(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Item not found"})),
            )
                .into_response(),
            AppError::ParsingError(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
            AppError::UnProcessableEntity { field, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "field": field, "message": message })),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        },
    }
}

pub async fn remove_stock_handler(
    State(state): State<SharedState>,
    Path(item_id): Path<Uuid>,
    Json(payload): Json<StockChange>,
) -> impl IntoResponse {
    match service::remove_stock(state, item_id, payload).await {
        Ok(item) => (StatusCode::OK, Json(item)).into_response(),
        Err(e) => match e {
            AppError::DatabaseError(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Item not found"})),
            )
                .into_response(),
            AppError::ParsingError(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
            AppError::UnProcessableEntity { field, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "field": field, "message": message })),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        },
    }
}

pub async fn list_movements_handler(State(state): State<SharedState>) -> impl IntoResponse {
    match crate::stock_movement::service::get_stock_movements(state).await {
        Ok(stock_movements) => (StatusCode::OK, Json(stock_movements)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
      