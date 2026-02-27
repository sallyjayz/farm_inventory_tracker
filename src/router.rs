use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

use crate::{
    app_state::SharedState, 
    farms::router::farms_router, 
    inventory::router::inventory_router, 
    stock_movement::router::stock_router
};

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .nest("/farms", farms_router(state.clone()))
        .nest("/items", inventory_router(state.clone()))
        .nest("/stock_movements", stock_router(state.clone()))
        .route("/server_checks", get(server_checks))
        .route("/", get(hello))
        .with_state(state)
}

async fn hello() -> &'static str {
    "Welcome to Farm Inventory Tracker API!"
}

async fn server_checks() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ok", "message": "Server is running" })))
}