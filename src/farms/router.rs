use std::sync::Arc;

use crate::app_state::{AppState, SharedState};
use crate::farms::handler::{create_farm_handler, get_farms_handler};
use axum::Router;
use axum::routing::get;

pub fn farms_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_farms_handler).post(create_farm_handler))
        .with_state(state)
}