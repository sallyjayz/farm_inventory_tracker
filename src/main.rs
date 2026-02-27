mod app;
mod app_state;
mod config;
mod errors;
mod router;

mod farms;
mod inventory;
mod stock_movement;

use app::start_app;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    start_app().await.unwrap();
}