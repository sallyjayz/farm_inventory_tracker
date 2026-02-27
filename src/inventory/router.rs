use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    app_state::{AppState, SharedState},
    inventory::handler::{
        create_item_handler, get_item_by_id_handler, get_items_handler,
    },
};

pub fn inventory_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_item_handler).get(get_items_handler))
        .route("/{id}", get(get_item_by_id_handler))
        .with_state(state)
}