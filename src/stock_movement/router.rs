use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    app_state::{AppState, SharedState},
    stock_movement::handler::{
        add_stock_handler, remove_stock_handler, list_movements_handler,
    },
};

pub fn stock_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}/add", post(add_stock_handler))
        .route("/{id}/remove", post(remove_stock_handler))
        .route("/", get(list_movements_handler))
        .with_state(state)
}