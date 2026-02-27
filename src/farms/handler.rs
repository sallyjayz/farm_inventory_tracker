use crate::app_state::SharedState;
use crate::farms::models::CreateFarm;
use crate::farms::service::create_farm;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

pub async fn get_farms_handler(State(state): State<SharedState>) -> impl IntoResponse {
    match crate::farms::service::get_farms(state).await {
        Ok(farms) => (StatusCode::OK, Json(farms)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn create_farm_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreateFarm>,
) -> impl IntoResponse {
    match create_farm(state, payload).await {
        Ok(farm) => (StatusCode::CREATED, Json(farm)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}