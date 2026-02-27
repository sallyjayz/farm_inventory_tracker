use axum::extract::{Path, Query, State};
use std::collections::HashMap;
use uuid::Uuid;

use crate::inventory::models::CreateInventoryItem;
use crate::inventory::service;
use crate::{app_state::SharedState, errors::AppError};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn create_item_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreateInventoryItem>,
) -> impl IntoResponse {
    match service::create_inventory_item(state, payload).await {
        Ok(item) => (StatusCode::CREATED, Json(item)).into_response(),
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
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        },
    }
}

pub async fn get_items_handler(
    State(state): State<SharedState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let farm_id = match params.get("farm_id").map(|s| Uuid::parse_str(s)) {
        Some(Ok(id)) => Some(id),
        Some(Err(e)) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"Invalid farm_id": e.to_string()})),
            )
                .into_response();
        }
        None => None,
    };



    match service::get_items(state, farm_id).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get_item_by_id_handler(
    State(state): State<SharedState>,
    Path(item_id): Path<String>,
) -> impl IntoResponse {
    match service::get_item_by_id(state, item_id).await {
        Ok(item) => Json(item).into_response(),
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
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        },
    }
}
